# ABOUTME: Makes the ingest scripts importable as modules for the test suite.
# ABOUTME: Adds the ingest directory to sys.path so `import rules_io` / `forbidden` resolve.
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
