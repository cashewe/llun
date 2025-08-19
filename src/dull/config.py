import os

from pydantic import BaseModel

from config_managers import (
    DullPyproject,
    Files,
    Rules,
    RuleRegistry,
)


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
