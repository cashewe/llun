from pathlib import Path
import tomllib
from typing import Any


class DullPyproject:
    """Manage pyproject.toml file.
    
    Notes
    -----
    This doesnt really scale to multiple pyprojects in a nest etc... does it
    do i care?
    probably not?
    but part of the point is to care too much right?
    """

    def __init__(self):
        self.config = {}

    def _load_pyproject(self):
        pyproject_path = Path("pyproject.toml")
        if pyproject_path.exists():
            with open(pyproject_path, "rb") as f:
                data = tomllib.load(f)
                self.config = data.get("tool", {}).get("dull", {})

    def get(self, el: str, default: Any) -> list[str] | None:
        """Thin wrapper on dict.get for if i change away from a dict."""
        return self.config.get(el, default)