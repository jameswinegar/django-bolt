use ahash::AHashMap;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString};

use std::sync::OnceLock;

use crate::router::parse_query_string;

/// Raw Rust data for deferred PyDict creation.
/// Only created when handler has `request` param and some components
/// weren't eagerly populated by typed params.
pub struct LazyRequestData {
    pub raw_query_string: Option<String>,
    pub raw_headers: Option<AHashMap<String, String>>,
    pub raw_cookies: Option<AHashMap<String, String>>,
    pub query_cache: OnceLock<Py<PyDict>>,
    pub headers_cache: OnceLock<Py<PyDict>>,
    pub cookies_cache: OnceLock<Py<PyDict>>,
}

/// Parse host:port from Actix's connection_info().host()
/// Returns (hostname, port_string) - port defaults to "443" for HTTPS, else "80"
#[inline]
fn parse_host_port<'a>(host: &'a str, scheme: &str) -> (&'a str, u16) {
    let default_port = if scheme.eq_ignore_ascii_case("https") {
        443
    } else {
        80
    };

    // IPv6 with brackets: [::1]:8080 or [::1]
    if let Some(bracket_end) = host.find(']') {
        // Check for port after closing bracket
        if host.len() > bracket_end + 2 && host.as_bytes()[bracket_end + 1] == b':' {
            if let Ok(port) = host[bracket_end + 2..].parse::<u16>() {
                return (&host[..bracket_end + 1], port);
            }
        }
        return (&host[..bracket_end + 1], default_port);
    }
    // IPv4/hostname: split on last colon
    match host.rsplit_once(':') {
        Some((name, port)) => match port.parse::<u16>() {
            Ok(parsed) => (name, parsed),
            Err(_) => (host, default_port),
        },
        _ => (host, default_port),
    }
}

#[pyclass]
pub struct PyRequest {
    pub method: String,
    pub path: String,
    pub body: Vec<u8>,
    /// Path parameters - None when no path params (saves 1 PyDict alloc per request)
    pub path_params: Option<Py<PyDict>>,
    /// Query parameters - None when no query params (saves 1 PyDict alloc per request)
    pub query_params: Option<Py<PyDict>>,
    /// Headers - None when handler doesn't need headers (saves 1 PyDict alloc per request)
    pub headers: Option<Py<PyDict>>,
    /// Cookies - None when handler doesn't need cookies (saves 1 PyDict alloc per request)
    pub cookies: Option<Py<PyDict>>,
    pub context: Option<Py<PyDict>>, // Middleware context data
    // None if no auth context or user not found
    pub user: Option<Py<PyAny>>,
    /// Lazy state dict for middleware AND dynamic attributes (e.g. _messages, _bolt_prebound_args).
    /// Only allocated on first access, so fast-path handlers with no prebound args pay zero cost.
    pub state: OnceLock<Py<PyDict>>,
    /// Form data - pre-typed by Rust. None when no form data (saves 1 PyDict alloc per request).
    pub form_map: Option<Py<PyDict>>,
    /// Files data. None when no file uploads (saves 1 PyDict alloc per request).
    pub files_map: Option<Py<PyDict>>,
    /// Lazy cached META dict for Django template compatibility
    /// Uses OnceLock for thread-safe lazy initialization (required by PyO3's pyclass)
    pub meta_cache: OnceLock<Py<PyDict>>,
    /// Lazy request data — only allocated when handler has `request` param
    /// and some components weren't eagerly populated by typed params.
    /// Boxed: null pointer (8 bytes) for the 99% of handlers without request param.
    pub lazy: Option<Box<LazyRequestData>>,
    /// Connection info from Actix's connection_info() - handles proxies correctly
    /// Host (may include port): "example.com:8080" or "[::1]:8080"
    pub conn_host: String,
    /// Scheme: "http" or "https" (from X-Forwarded-Proto or request)
    pub conn_scheme: String,
    /// Remote address: client IP (from X-Forwarded-For, X-Real-IP, or peer)
    pub conn_remote_addr: String,
}

/// Private resolve helpers — check eager field first, then lazy raw data.
/// Returns a usable PyDict for the component, or None when no data at all.
impl PyRequest {
    /// Resolve headers: eager `self.headers` → lazy `raw_headers` → None.
    #[inline]
    fn resolve_headers(&self, py: Python<'_>) -> Py<PyDict> {
        if let Some(d) = &self.headers {
            return d.clone_ref(py);
        }
        if let Some(lazy) = &self.lazy {
            if let Some(raw) = &lazy.raw_headers {
                return lazy
                    .headers_cache
                    .get_or_init(|| {
                        let dict = PyDict::new(py);
                        for (k, v) in raw {
                            let _ = dict.set_item(k.as_str(), v.as_str());
                        }
                        dict.unbind()
                    })
                    .clone_ref(py);
            }
        }
        PyDict::new(py).unbind()
    }

    /// Resolve cookies: eager `self.cookies` → lazy `raw_cookies` → None.
    #[inline]
    fn resolve_cookies(&self, py: Python<'_>) -> Py<PyDict> {
        if let Some(d) = &self.cookies {
            return d.clone_ref(py);
        }
        if let Some(lazy) = &self.lazy {
            if let Some(raw) = &lazy.raw_cookies {
                return lazy
                    .cookies_cache
                    .get_or_init(|| {
                        let dict = PyDict::new(py);
                        for (k, v) in raw {
                            let _ = dict.set_item(k.as_str(), v.as_str());
                        }
                        dict.unbind()
                    })
                    .clone_ref(py);
            }
        }
        PyDict::new(py).unbind()
    }

    /// Resolve query params: eager `self.query_params` → lazy `raw_query_string` → None.
    #[inline]
    fn resolve_query(&self, py: Python<'_>) -> Py<PyDict> {
        if let Some(d) = &self.query_params {
            return d.clone_ref(py);
        }
        if let Some(lazy) = &self.lazy {
            if let Some(raw_qs) = &lazy.raw_query_string {
                return lazy
                    .query_cache
                    .get_or_init(|| {
                        let parsed = parse_query_string(raw_qs);
                        let dict = PyDict::new(py);
                        for (k, v) in &parsed {
                            let _ = dict.set_item(k.as_str(), v.as_str());
                        }
                        dict.unbind()
                    })
                    .clone_ref(py);
            }
        }
        PyDict::new(py).unbind()
    }

    /// Resolve headers as Option — for META/build_absolute_uri that need to check presence.
    #[inline]
    fn resolve_headers_opt(&self, py: Python<'_>) -> Option<Py<PyDict>> {
        if let Some(d) = &self.headers {
            return Some(d.clone_ref(py));
        }
        if let Some(lazy) = &self.lazy {
            if lazy.raw_headers.is_some() {
                return Some(self.resolve_headers(py));
            }
        }
        None
    }

    /// Resolve query as Option — for get_full_path/build_absolute_uri that check presence.
    #[inline]
    fn resolve_query_opt(&self, py: Python<'_>) -> Option<Py<PyDict>> {
        if let Some(d) = &self.query_params {
            return Some(d.clone_ref(py));
        }
        if let Some(lazy) = &self.lazy {
            if lazy.raw_query_string.is_some() {
                return Some(self.resolve_query(py));
            }
        }
        None
    }

    /// Get raw query string — for META's QUERY_STRING (perfect fidelity).
    #[inline]
    fn raw_query_string_for_meta(&self) -> Option<&str> {
        if let Some(lazy) = &self.lazy {
            if let Some(qs) = &lazy.raw_query_string {
                return Some(qs.as_str());
            }
        }
        None
    }
}

#[pymethods]
impl PyRequest {
    /// OPTIMIZATION: #[inline] on hot path getters
    #[getter]
    #[inline]
    fn method(&self) -> &str {
        &self.method
    }

    #[getter]
    #[inline]
    fn path(&self) -> &str {
        &self.path
    }

    #[getter]
    #[inline]
    fn body<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        PyBytes::new(py, &self.body).into_any().unbind()
    }

    #[getter]
    #[inline]
    fn context<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        match &self.context {
            Some(ctx) => ctx.clone_ref(py).into_any(),
            None => py.None(),
        }
    }

    /// Get the user object (LazyUser proxy set by Python's _dispatch).
    ///
    /// Returns a LazyUser proxy that loads the user from the database
    /// on first access (no await needed in handler code).
    ///
    /// Returns:
    /// - LazyUser proxy if authentication succeeded
    /// - None if no auth context or authentication failed
    #[getter]
    fn user<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        match &self.user {
            Some(user) => user.clone_ref(py),
            None => py.None(),
        }
    }

    /// Set the user object (called by Django middleware via DjangoMiddlewareStack).
    ///
    /// This allows Django's AuthenticationMiddleware to set request.user
    /// just like in standard Django.
    #[setter]
    fn set_user(&mut self, value: Py<PyAny>) {
        self.user = Some(value);
    }

    /// Get headers as a dict for middleware access.
    /// Values are pre-typed by Rust (int, float, bool, str) on the eager path.
    /// On the lazy path (request param), values are raw strings.
    ///
    /// Example:
    ///     auth_header = request.headers.get("authorization")
    #[getter]
    #[inline]
    fn headers<'py>(&self, py: Python<'py>) -> Py<PyDict> {
        self.resolve_headers(py)
    }

    /// Get cookies as a dict for middleware access.
    /// Values are pre-typed by Rust (int, float, bool, str) on the eager path.
    /// On the lazy path (request param), values are raw strings.
    ///
    /// Example:
    ///     session_id = request.cookies.get("session_id")
    #[getter]
    #[inline]
    fn cookies<'py>(&self, py: Python<'py>) -> Py<PyDict> {
        self.resolve_cookies(py)
    }

    /// Get query params as a dict for middleware access.
    /// Values are pre-typed by Rust (int, float, bool, str) on the eager path.
    /// On the lazy path (request param), values are raw strings.
    ///
    /// Example:
    ///     page = request.query.get("page", 1)  # Returns int directly
    #[getter]
    #[inline]
    fn query<'py>(&self, py: Python<'py>) -> Py<PyDict> {
        self.resolve_query(py)
    }

    /// Get the state dict for middleware to store arbitrary data.
    ///
    /// This follows the Starlette pattern where middleware can store
    /// request-scoped data that persists through the request lifecycle.
    ///
    /// Example:
    ///     request.state["request_id"] = "abc123"
    ///     request.state["tenant"] = tenant_obj
    #[getter]
    #[inline]
    fn state<'py>(&self, py: Python<'py>) -> Py<PyDict> {
        self.get_or_init_state(py)
    }

    /// Internal: get or lazily create the state dict (OnceLock - zero cost when never accessed).
    #[inline]
    fn get_or_init_state(&self, py: Python<'_>) -> Py<PyDict> {
        if let Some(s) = self.state.get() {
            return s.clone_ref(py);
        }
        let new_dict = PyDict::new(py).unbind();
        // OnceLock::set returns Err if already set (harmless race — each request owns its object)
        match self.state.set(new_dict.clone_ref(py)) {
            Ok(()) => new_dict,
            Err(_) => self.state.get().unwrap().clone_ref(py),
        }
    }

    /// Get form data as a dict for parameter access.
    /// Values are pre-typed by Rust (int, float, bool, str).
    ///
    /// Example:
    ///     username = request.form.get("username")
    ///     age = request.form.get("age")  # Returns int directly
    #[getter]
    #[inline]
    fn form<'py>(&self, py: Python<'py>) -> Py<PyDict> {
        match &self.form_map {
            Some(d) => d.clone_ref(py),
            None => PyDict::new(py).unbind(),
        }
    }

    /// Get files as a dict for file access.
    /// Each file entry contains: filename, content, content_type, size, temp_path (if spooled to disk).
    ///
    /// Example:
    ///     avatar = request.files.get("avatar")  # {"filename": "photo.jpg", ...}
    #[getter]
    #[inline]
    fn files<'py>(&self, py: Python<'py>) -> Py<PyDict> {
        match &self.files_map {
            Some(d) => d.clone_ref(py),
            None => PyDict::new(py).unbind(),
        }
    }

    /// Get META dict for Django template compatibility.
    ///
    /// This provides a Django HttpRequest-compatible META dict containing:
    /// - REQUEST_METHOD: HTTP method
    /// - PATH_INFO: Request path
    /// - QUERY_STRING: Query string
    /// - HTTP_*: Headers converted to META format (e.g., host -> HTTP_HOST)
    /// - CONTENT_TYPE: Content-Type header (without HTTP_ prefix)
    /// - CONTENT_LENGTH: Content-Length header (without HTTP_ prefix)
    ///
    /// The dict is lazily built on first access and cached for subsequent accesses.
    /// This ensures zero overhead for API handlers that don't use templates.
    ///
    /// Example:
    ///     method = request.META.get("REQUEST_METHOD")  # "GET"
    ///     host = request.META.get("HTTP_HOST")  # "example.com"
    #[getter]
    #[allow(non_snake_case)]
    fn META<'py>(&self, py: Python<'py>) -> PyResult<Py<PyDict>> {
        // If Django middleware synced a META dict into request.state, reuse it.
        // This keeps template-time CSRF token generation aligned with middleware validation.
        let state_py = self.get_or_init_state(py);
        let state_dict = state_py.bind(py);
        if let Some(state_meta) = state_dict.get_item("META")? {
            if let Ok(state_meta_dict) = state_meta.cast::<PyDict>() {
                let meta_unbind = state_meta_dict.clone().unbind();
                let _ = self.meta_cache.set(meta_unbind.clone_ref(py));
                return Ok(meta_unbind);
            }
        }

        // Return cached if already built (zero-overhead check)
        if let Some(meta) = self.meta_cache.get() {
            return Ok(meta.clone_ref(py));
        }

        // Build META dict only on first access
        let meta = PyDict::new(py);

        // Standard META keys (Django HttpRequest compatible)
        meta.set_item("REQUEST_METHOD", &self.method)?;
        meta.set_item("PATH_INFO", &self.path)?;

        // QUERY_STRING - use raw query string when available (perfect fidelity),
        // otherwise reconstruct from parsed query_params
        let query_string = if let Some(raw_qs) = self.raw_query_string_for_meta() {
            raw_qs.to_string()
        } else {
            match &self.query_params {
                Some(qp) => {
                    let query_dict = qp.bind(py);
                    if query_dict.is_empty() {
                        String::new()
                    } else {
                        query_dict
                            .iter()
                            .filter_map(|(k, v)| {
                                let key = k.extract::<String>().ok()?;
                                let val = v.str().ok()?.to_string();
                                Some(format!("{}={}", key, val))
                            })
                            .collect::<Vec<_>>()
                            .join("&")
                    }
                }
                None => String::new(),
            }
        };
        meta.set_item("QUERY_STRING", query_string)?;

        // Server info from Actix's connection_info() - handles IPv6 and proxies correctly
        // conn_host may include port: "example.com:8080" or "[::1]:8080"
        let (server_name, server_port) = parse_host_port(&self.conn_host, &self.conn_scheme);
        meta.set_item("SERVER_NAME", server_name)?;
        meta.set_item("SERVER_PORT", server_port.to_string())?;
        meta.set_item("SERVER_PROTOCOL", "HTTP/1.1")?;

        // REMOTE_ADDR from Actix's connection_info().realip_remote_addr()
        // Actix handles X-Forwarded-For, Forwarded header, and peer_addr fallback
        meta.set_item("REMOTE_ADDR", &self.conn_remote_addr)?;
        meta.set_item("REMOTE_HOST", &self.conn_remote_addr)?;

        // SCRIPT_NAME is usually empty for Django apps
        meta.set_item("SCRIPT_NAME", "")?;

        // Convert headers to HTTP_* format
        // Header keys are already lowercase (normalized by http crate)
        if let Some(headers_py) = self.resolve_headers_opt(py).as_ref() {
            for (key, value) in headers_py.bind(py).iter() {
                if let (Ok(k), Ok(v)) = (key.extract::<String>(), value.extract::<String>()) {
                    let meta_key = if k == "content-type" {
                        "CONTENT_TYPE".to_string()
                    } else if k == "content-length" {
                        "CONTENT_LENGTH".to_string()
                    } else {
                        format!("HTTP_{}", k.to_uppercase().replace('-', "_"))
                    };
                    meta.set_item(meta_key, v)?;
                }
            }
        }

        // Cache and return
        let meta_unbind = meta.unbind();
        let _ = self.meta_cache.set(meta_unbind.clone_ref(py));
        Ok(meta_unbind)
    }

    /// Get the async user loader (Django-style).
    ///
    /// Returns the async user callable set by Django's AuthenticationMiddleware.
    /// Use this in async handlers to load the user without blocking:
    ///
    ///     user = await request.auser()
    ///
    /// This follows Django's pattern where `request.auser` is an async callable
    /// that loads the user from the database asynchronously.
    ///
    /// Returns:
    ///     Async callable that returns the user when awaited.
    ///     If Django middleware is not configured, returns a callable that
    ///     returns AnonymousUser (matching Django's behavior).
    #[getter]
    fn auser<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        // Get "auser" from state dict (set by Django middleware adapter)
        let state_py = self.get_or_init_state(py);
        let state_dict = state_py.bind(py);
        match state_dict.get_item("auser") {
            Ok(Some(auser)) => Ok(auser.unbind()),
            _ => {
                // Return async callable that returns AnonymousUser
                // This matches Django's behavior when AuthenticationMiddleware isn't configured
                let django_bolt_module = py.import("django_bolt.auth.anonymous")?;
                let auser_fallback = django_bolt_module.getattr("auser_fallback")?;
                Ok(auser_fallback.unbind())
            }
        }
    }

    /// Set the async user loader (Django-style).
    ///
    /// This allows Django's alogin()/alogout() functions to set request.auser
    /// just like in standard Django.
    #[setter]
    fn set_auser(&mut self, py: Python<'_>, value: Py<PyAny>) -> PyResult<()> {
        let state_py = self.get_or_init_state(py);
        state_py.bind(py).set_item("auser", value)?;
        Ok(())
    }

    /// Get the full path with query string (Django-compatible).
    ///
    /// Example:
    ///     /users?page=2&limit=10
    ///
    /// This matches Django's HttpRequest.get_full_path() method.
    fn get_full_path(&self, py: Python<'_>) -> String {
        // Use raw query string for perfect fidelity when available
        if let Some(raw_qs) = self.raw_query_string_for_meta() {
            if raw_qs.is_empty() {
                return self.path.clone();
            }
            return format!("{}?{}", self.path, raw_qs);
        }
        match self.resolve_query_opt(py) {
            Some(qp) => {
                let query_dict = qp.bind(py);
                if query_dict.is_empty() {
                    self.path.clone()
                } else {
                    let query_string: String = query_dict
                        .iter()
                        .filter_map(|(k, v)| {
                            let key = k.extract::<String>().ok()?;
                            let val = v.str().ok()?.to_string();
                            Some(format!("{}={}", key, val))
                        })
                        .collect::<Vec<_>>()
                        .join("&");
                    format!("{}?{}", self.path, query_string)
                }
            }
            None => self.path.clone(),
        }
    }

    /// Build absolute URI (Django-compatible).
    ///
    /// Example:
    ///     http://example.com/users?page=2
    ///
    /// This matches Django's HttpRequest.build_absolute_uri() method.
    /// Uses Host header to determine the scheme and host.
    #[pyo3(signature = (location=None))]
    fn build_absolute_uri(&self, py: Python<'_>, location: Option<&str>) -> String {
        let (host, scheme) = match self.resolve_headers_opt(py) {
            Some(headers_py) => {
                let headers_dict = headers_py.bind(py);
                let h = headers_dict
                    .get_item("host")
                    .ok()
                    .flatten()
                    .and_then(|v| v.extract::<String>().ok())
                    .unwrap_or_else(|| "localhost".to_string());
                let s = headers_dict
                    .get_item("x-forwarded-proto")
                    .ok()
                    .flatten()
                    .and_then(|v| v.extract::<String>().ok())
                    .unwrap_or_else(|| "http".to_string());
                (h, s)
            }
            None => ("localhost".to_string(), "http".to_string()),
        };

        let path = location.unwrap_or(&self.path);

        // Use raw query string for perfect fidelity when available
        if location.is_none() {
            if let Some(raw_qs) = self.raw_query_string_for_meta() {
                if !raw_qs.is_empty() {
                    return format!("{}://{}{}?{}", scheme, host, path, raw_qs);
                }
                return format!("{}://{}{}", scheme, host, path);
            }
        }

        // Build query string from query_params if using current path
        let has_query = match self.resolve_query_opt(py) {
            Some(ref qp) if location.is_none() => !qp.bind(py).is_empty(),
            _ => false,
        };
        if !has_query {
            format!("{}://{}{}", scheme, host, path)
        } else {
            let query_py = self.resolve_query(py);
            let query_dict = query_py.bind(py);
            let query_string: String = query_dict
                .iter()
                .filter_map(|(k, v)| {
                    let key = k.extract::<String>().ok()?;
                    let val = v.str().ok()?.to_string();
                    Some(format!("{}={}", key, val))
                })
                .collect::<Vec<_>>()
                .join("&");
            format!("{}://{}{}?{}", scheme, host, path, query_string)
        }
    }

    #[pyo3(signature = (key, /, default=None))]
    fn get<'py>(&self, py: Python<'py>, key: &str, default: Option<Py<PyAny>>) -> Py<PyAny> {
        match key {
            "method" => PyString::new(py, &self.method).into_any().unbind(),
            "path" => PyString::new(py, &self.path).into_any().unbind(),
            "body" => PyBytes::new(py, &self.body).into_any().unbind(),
            "params" => match &self.path_params {
                Some(d) => d.clone_ref(py).into_any(),
                None => PyDict::new(py).into_any().unbind(),
            },
            "query" => self.resolve_query(py).into_any(),
            "headers" => self.resolve_headers(py).into_any(),
            "cookies" => self.resolve_cookies(py).into_any(),
            "state" => self.get_or_init_state(py).into_any(),
            "auth" | "context" => match &self.context {
                Some(ctx) => ctx.clone_ref(py).into_any(),
                None => default.unwrap_or_else(|| py.None()),
            },
            _ => default.unwrap_or_else(|| py.None()),
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, key: &str) -> PyResult<Py<PyAny>> {
        match key {
            "method" => Ok(PyString::new(py, &self.method).into_any().unbind()),
            "path" => Ok(PyString::new(py, &self.path).into_any().unbind()),
            "body" => Ok(PyBytes::new(py, &self.body).into_any().unbind()),
            "params" => Ok(match &self.path_params {
                Some(d) => d.clone_ref(py).into_any(),
                None => PyDict::new(py).into_any().unbind(),
            }),
            "query" => Ok(self.resolve_query(py).into_any()),
            "headers" => Ok(self.resolve_headers(py).into_any()),
            "cookies" => Ok(self.resolve_cookies(py).into_any()),
            "state" => Ok(self.get_or_init_state(py).into_any()),
            "context" => Ok(match &self.context {
                Some(ctx) => ctx.clone_ref(py).into_any(),
                None => py.None(),
            }),
            _ => Err(pyo3::exceptions::PyKeyError::new_err(key.to_string())),
        }
    }

    #[pyo3(signature = (key, /, default=None))]
    fn setdefault<'py>(
        &self,
        py: Python<'py>,
        key: &str,
        default: Option<Py<PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        if key == "state" {
            return Ok(self.get_or_init_state(py).into_any());
        }
        Ok(self.get(py, key, default))
    }

    fn __setitem__(&mut self, key: &str, value: Py<PyAny>) -> PyResult<()> {
        match key {
            "user" => {
                // Allow Python's _dispatch to set LazyUser proxy (loads user on first access)
                self.user = Some(value);
                Ok(())
            }
            _ => Err(pyo3::exceptions::PyKeyError::new_err(key.to_string())),
        }
    }

    /// Get unknown attributes from state dict.
    ///
    /// This enables Django middleware to read arbitrary attributes on the request
    /// object (e.g., request._messages) which are stored in the state dict.
    /// Note: __getattr__ is only called when attribute is NOT found via normal lookup.
    ///
    /// Example:
    ///     messages = request._messages  # Reads from state["_messages"]
    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        let state_py = self.get_or_init_state(py);
        let state_dict = state_py.bind(py);
        match state_dict.get_item(name)? {
            Some(value) => Ok(value.unbind()),
            None => Err(pyo3::exceptions::PyAttributeError::new_err(format!(
                "'Request' object has no attribute '{}'",
                name
            ))),
        }
    }
}
