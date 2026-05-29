# ABOUTME: Type stubs for the maturin-built vacant._core extension.
# ABOUTME: Mirrors the PyO3 surface in crates/vacant-pyext/src/lib.rs.

from os import PathLike
from typing import Any

_PathLike = str | PathLike[str]

def load_rules(path: _PathLike) -> None: ...
def check_many(
    domains: list[str],
    *,
    concurrency: int = 64,
    timeout: float = 4.0,
    cache: DiskCache | None = None,
    cache_ttl: float = 86_400.0,
    verify: bool = False,
) -> list[dict[str, Any]]: ...

class DiskCache:
    def __init__(self, path: _PathLike | None = None) -> None: ...
    @staticmethod
    def default_path() -> str: ...
    def get(self, domain: str, ttl: float) -> dict[str, Any] | None: ...
    def put(self, domain: str, zone: str, status: str, detail: str) -> None: ...
