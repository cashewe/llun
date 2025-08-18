from pathlib import Path
import tomllib
from typing import Any

from pydantic import BaseModel

from dull._types import Rule


class RuleRegistry:
    """Manages the collection of available strategic rules."""
    
    def __init__(self) -> None:
        self.rules: dict[str, Rule] = {}
        self._load_rules()
        self.default_rules = self._get_default_rules()
    
    def _load_rules(self) -> None:
        """Load the default set of rules into self.rules."""
        rules_folder = Path("rules")
        for filename in rules_folder.glob("*.json"):            
            with filename.open("r", encoding="utf-8") as f:
                data = json.load(f)
            
            self.rules[filename.stem] = Rule(
                brief_description=data["brief_description"],
                long_description=data["long_description"],
                example=data["example"]
            )

    def _get_default_rules() -> list[str]:
        """Load the default rule list."""
        path = Path("data/default_rules.txt")

        with path.open("r", encoding="utf-8") as f:
            return [line.strip() for line in f if line.strip()]
    
    def get_rules(self, rule_codes: list[str] | None) -> list[Rule]:
        """Get rules by their codes."""
        if not rule_codes:
            rule_codes = self.default_rules
        
        return [self.rules[code] for code in rule_codes if code in self.rules]


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