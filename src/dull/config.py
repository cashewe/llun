import os

from pydantic import BaseModel

from dull.config_managers import (
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
        rule_registry: RuleRegistry | None = None,
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
        rules = self.rule_registry.get_rules(rules or None)

        if not files:
            files = self.raw_config.get("files", ["."])
        files = Files(paths=files)

        api_key = os.getenv("OPENAI_API_KEY") or self.raw_config.get("openai_api_key", "no key!")

        return Config(
            rules=rules,
            files=files,
            api_key=api_key
        )
