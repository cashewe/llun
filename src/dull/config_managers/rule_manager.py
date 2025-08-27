from importlib import resources
import json
from pathlib import Path

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
        return "/n".join([str(rule) for rule in self.rules])


class RuleRegistry:
    """Manages the collection of available strategic rules."""
    
    def __init__(self) -> None:
        self.rules_dict: dict[str, Rule] = {}
        self._load_rules()

        self.default_rules: list[str] = []
        self._get_default_rules()
    
    def _load_rules(self) -> None:
        """Load the default set of rules into self.rules."""
        package = "dull.rules"  # adjust if your JSON files are in a different subpackage

        # Iterate through all JSON resources in the package
        for entry in resources.files(package).iterdir():
            if entry.suffix == ".json":
                with entry.open("r", encoding="utf-8") as f:
                    data = json.load(f)

                self.rules_dict[entry.stem] = Rule(
                    brief_description=data["brief_description"],
                    long_description=data["long_description"],
                    example=data["example"]
                )

    def _get_default_rules(self) -> None:
        """Load the default rule list."""
        package = "dull.data"

        with resources.files(package).joinpath("default_rules.txt").open("r", encoding="utf-8") as f:
            self.default_rules = [line.strip() for line in f if line.strip()]
    
    def get_rules(self, rule_codes: list[str] | None) -> Rules:
        """Get rules by their codes."""
        if not rule_codes:
            rule_codes = self.default_rules
        
        return Rules(
            rules=[self.rules_dict[code] for code in rule_codes if code in self.rules_dict]
        )