"""Tests for injector return type contracts.

The Rust WebSocket dispatcher casts injector args to PyList. If an injector
returns a tuple instead, the cast fails at runtime but the Python test client
silently masks this via `list(args)`. These tests enforce the contract at the
Python level so type mismatches are caught before they reach Rust.

Regression test for #172.
"""

from __future__ import annotations

from typing import Any

from django_bolt._kwargs.model import (
    HandlerPattern,
    _NO_PARAMS,
    _injector_no_params,
    compile_argument_injector,
    compile_binder,
)


class TestNoParamsInjector:
    """Verify _NO_PARAMS and _injector_no_params return lists, not tuples."""

    def test_no_params_args_is_list(self):
        args, kwargs = _NO_PARAMS
        assert isinstance(args, list), (
            f"_NO_PARAMS args must be a list for Rust PyList cast, got {type(args).__name__}"
        )

    def test_no_params_kwargs_is_dict(self):
        args, kwargs = _NO_PARAMS
        assert isinstance(kwargs, dict)

    def test_injector_no_params_returns_list_args(self):
        args, kwargs = _injector_no_params(None)
        assert isinstance(args, list), (
            f"_injector_no_params must return list args for Rust PyList cast, got {type(args).__name__}"
        )


class TestCompiledInjectorTypes:
    """Verify all compiled injectors return list args."""

    def _make_meta(self, pattern: HandlerPattern) -> dict[str, Any]:
        """Create minimal handler metadata for injector compilation."""
        return {
            "fields": [],
            "path_params": set(),
            "http_method": "WEBSOCKET",
            "handler_pattern": pattern,
            "needs_body": False,
            "needs_query": False,
            "needs_headers": False,
            "needs_cookies": False,
            "needs_path_params": False,
            "needs_form_parsing": False,
            "mode": "websocket_only",
        }

    def test_no_params_pattern_returns_list(self):
        meta = self._make_meta(HandlerPattern.NO_PARAMS)
        injector = compile_argument_injector(meta, {}, compile_binder)
        args, kwargs = injector({"path_params": {}, "query_params": {}})
        assert isinstance(args, list), (
            f"NO_PARAMS injector must return list args, got {type(args).__name__}"
        )
