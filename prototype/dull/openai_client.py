from importlib import resources
import json
from typing import Any

import openai

from dull.config import Config


class PromptManager:
    def __init__(
        self,
        config: Config,
        context: str,
    ):
        self.system_prompt = PromptManager._load_system_prompt()
        self.user_prompt = PromptManager._load_user_prompt(config, context)

    @staticmethod
    def _load_system_prompt() -> str:
        package = "dull.data"

        schema_path = resources.files(package).joinpath("response_schema.json")
        with schema_path.open("r", encoding="utf-8") as f:
            schema = json.load(f)
        formatted_schema = json.dumps(schema, indent=2)

        system_prompt_path = resources.files(package).joinpath("system_prompt.txt")
        with system_prompt_path.open("r", encoding="utf-8") as f:
            prompt = f.read()
        return prompt.format(formatted_schema=formatted_schema)

    @staticmethod
    def _load_user_prompt(
        config: Config,
        context: str,
    ) -> str:
        package = "dull.data"

        user_prompt_path = resources.files(package).joinpath("user_prompt.txt")
        with user_prompt_path.open("r", encoding="utf-8") as f:
            return f.read().format(
                rules=str(config.rules),
                context=context,
                code=str(config.files),
            )

    def get_prompts(self) -> list[dict[str, str]]:
        return [
            {"role": "system", "content": self.system_prompt},
            {"role": "user", "content": self.user_prompt},
        ]


class OpenAiClient:
    """Handles communication with the LLM service."""

    def __init__(
        self,
        config: Config,
        context: str,
    ):
        self.client = openai.OpenAI(api_key=config.api_key)
        self.prompt_manager = PromptManager(config, context)

    def lint_code(self) -> dict[str, Any]:
        """Send code and rules to LLM for analysis."""

        response = self.client.chat.completions.create(
            model="gpt-4", messages=self.prompt_manager.get_prompts(), temperature=0.1
        )

        try:
            return json.loads(response.choices[0].message.content)
        except json.JSONDecodeError:
            return {
                "error": "Failed to parse LLM response as JSON",
                "raw_response": response.choices[0].message.content,
            }
