# ABOUTME: Placeholder package reserving the PyPI name `vacant`.
# ABOUTME: The real tool is a Rust CLI; install via `brew install alltuner/tap/vacant` or `cargo install vacant`.

__version__ = "0.0.1"

_NOTICE = (
    "The PyPI name 'vacant' is reserved for the Rust CLI of the same name.\n"
    "Install it with one of:\n"
    "    brew install alltuner/tap/vacant\n"
    "    cargo install vacant\n"
    "See https://github.com/alltuner/vacant"
)


def notice() -> str:
    return _NOTICE
