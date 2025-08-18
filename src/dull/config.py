from pathlib import Path
import tomllib
from typing import Any

from pydantic import BaseModel


def load_config() -> dict[str, Any]:
    """Load configuration from available config files.

    I'm going by convention that the config files will be either
    pyproject.toml or dull.toml files.
    """
    config = {}
    pyproject_path = Path("pyproject.toml")
    if pyproject_path.exists():
        with open(pyproject_path, "rb") as f:
            data = tomllib.load(f)
            config = data.get("tool", {}).get("dull", {})
    
    else:
        dull_path = Path("dull.toml")
        if dull_path.exists():
            with open(dull_path, "rb") as f:
                config = tomllib.load(f)
    
    return config


def get_files(file_patterns: list[str]):
    """Get the list of files the user has requested."""
    file_list = []

    for pattern in file_patterns:
        if pattern == ".":
            return Path(".").rglob("*.py")  # we *DO NOT* lint .doc :(
        
        elif Path(pattern).exists():
            file_list.append(Path(pattern))
        
        else:
            raise FileNotFoundError("You've gone and asked for a file you haven't even provided. gimp.")


class Config(BaseModel):
    """maybe the above becomes a from dict method here?
    am i over cooking this POC?
    do i care if i am?
    """
    ...