import json
import os
from pathlib import Path
import tomllib
from typing import Any

from pydantic import BaseModel

class Rule(BaseModel):
    """An architectural rule to be enforced."""
    brief_description: str
    long_description: str
    example: str

    def __str__(self) -> str:
        return f"#{self.brief_description}\n*{self.long_description}*\n**Example**: {self.example}"


class Rules(BaseModel):  # why does pydantic RootModel exist? seems to add nothing but complexity?
    rules: list[Rule]

    def __str__(self) -> str:
        return "/n".join([rule for rule in self.rules])


class RuleRegistry:
    """Manages the collection of available strategic rules."""
    
    def __init__(self) -> None:
        self.rules_dict: dict[str, Rule] = {}
        self._load_rules()

        self.default_rules: list[str] = []
        self._get_default_rules()
    
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

    def _get_default_rules(self) -> None:
        """Load the default rule list."""
        path = Path("data/default_rules.txt")

        with path.open("r", encoding="utf-8") as f:
            self.default_rules = [line.strip() for line in f if line.strip()]
    
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


class Files:
    """Manages the file requests the user makes."""

    def __init__(
        self,
        paths: list[str] = ["."]
    ):
        self.files: list[File] = []
        resolved_paths = Files._get_paths(paths)
        self._get_files(resolved_paths)

    @staticmethod
    def _get_paths(file_patterns: list[str]) -> list[Path]:
        """Get the list of files the user has requested and validate they exist."""
        file_list = []

        for pattern in file_patterns:
            if pattern == ".":
                return list(Path(".").rglob("*.py"))  # by default, run exclusively on .py files to save tokens.
            
            elif Path(pattern).exists():
                file_list.append(Path(pattern))
            
            else:
                raise FileNotFoundError("You've gone and asked for a file you haven't even provided. gimp.")

        return file_list

    def _get_files(self, paths: list[Path]) -> None:
        for path in paths:
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()

            self.files.append(
                File(
                    name=str(path),
                    content=content,
                )
            )

    def __str__(self) -> str:
        return "\n".join([str(file) for file in self.files])
    

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


class Config(BaseModel):
    rules: Rules
    files: Files
    api_key: str


class ConfigFactory:
    """Create config for the app."""

    def __init__(
        self,
        rule_registry: RuleRegistry | None,
    ):
        self.raw_config = DullPyproject()
        self.rule_registry = rule_registry or RuleRegistry()

    def create_config(
        self,
        rules: list[str],
        files: list[str],
    ) -> Config:
        if not rules:
            rules = self.raw_config.get("rules", [])
        self.rules = self.rule_registry.get_rules(rules or None)

        if not files:
            files = self.raw_config.get("files", ["."])
        self.files = Files(paths=files)

        api_key = self.raw_config.get("openai_api_key") or os.getenv("OPENAI_API_KEY")

        return Config(
            rules=rules,
            files=files,
            api_key=api_key
        )
