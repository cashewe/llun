import json
from pathlib import Path

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
