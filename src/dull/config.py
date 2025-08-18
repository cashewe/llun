import json
import os
from pathlib import Path
import tomllib
from typing import Any, Optional

from pydantic import BaseModel


class Rule(BaseModel):
    """An architectural rule to be enforced."""
    brief_description: str
    long_description: str
    example: str

    def __str__(self) -> str:
        return f"#{self.brief_description}\n*{self.long_description}*\n**Example**: {self.example}"


class Rules(BaseModel):
    rules: list[Rule]

    def __str__(self) -> str:
        return "/n".join([rule for rule in self.rules])


class RuleRegistry:
    """Manages the collection of available strategic rules."""
    
    def __init__(self) -> None:
        self.rules_dict: dict[str, Rule] = {}
        self._load_rules()
        self.default_rules = self._get_default_rules()
    
    def _load_rules(self) -> None:
        """Load the default set of rules into self.rules."""
        rules_folder = Path("rules")
        for filename in rules_folder.glob("*.json"):            
            with filename.open("r", encoding="utf-8") as f:
                data = json.load(f)
            
            self.rules_dict[filename.stem] = Rule(
                brief_description=data["brief_description"],
                long_description=data["long_description"],
                example=data["example"]
            )

    def _get_default_rules(self) -> list[str]:
        """Load the default rule list."""
        path = Path("data/default_rules.txt")

        with path.open("r", encoding="utf-8") as f:
            return [line.strip() for line in f if line.strip()]
    
    def get_rules(self, rule_codes: list[str] | None) -> Rules:
        """Get rules by their codes."""
        if not rule_codes:
            rule_codes = self.default_rules
        
        return Rules(
            rules=[self.rules_dict[code] for code in rule_codes if code in self.rules_dict]
        )


class File(BaseModel):
    """Representation of a file."""
    name: str
    content: str

    def __str__(self) -> str:
        return f"#{self.name}\n{self.content}"


class Files(BaseModel):
    files: list[File]

    def __str__(self) -> str:
        return "\n".join([file for file in self.files])


class Config:
    """Config for the app."""

    def __init__(
        self,
        rule_registry: RuleRegistry | None
    ):
        raw_config = Config._load()
        self.rule_registry = rule_registry or RuleRegistry()

        rule_codes = raw_config.get("rules", [])
        self.rules = rule_registry.get_rules(rule_codes or None)

        file_patterns = raw_config.get("files", ["."])
        self.files = Config._get_files(file_patterns)

        self.api_key = raw_config.get("openai_api_key") or os.getenv("OPENAI_API_KEY")

    @staticmethod
    def _load() -> dict[str, Any]:
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

    @staticmethod
    def _get_files(file_patterns: list[str]):
        """Get the list of files the user has requested."""
        file_list = []

        for pattern in file_patterns:
            if pattern == ".":
                return list(Path(".").rglob("*.py"))  # we *DO NOT* lint .doc :(
            
            elif Path(pattern).exists():
                file_list.append(Path(pattern))
            
            else:
                raise FileNotFoundError("You've gone and asked for a file you haven't even provided. gimp.")
