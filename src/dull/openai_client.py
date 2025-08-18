import json
from pathlib import Path
import sys
from typing import Any

import openai

from config import Config
from src.dull.data.prompts import system_prompt, user_prompt


class OpenAiClient:
    """Handles communication with the LLM service."""
    
    def __init__(
        self,
        config: Config,
        context: str,
    ):
        self.client = openai.OpenAI(api_key=config.api_key)
        self.system_prompt = system_prompt
        self.user_prompt = user_prompt.format(
            rules=str(config.rules),
            context=context,
            code=str(config.files),
        )

    def lint_code(self) -> dict[str, Any]:
        """Send code and rules to LLM for analysis."""        
        
        response = self.client.chat.completions.create(
            model="gpt-4",
            messages=[
                {"role": "system", "content": self.system_prompt},
                {"role": "user", "content": self.user_prompt},
            ],
            temperature=0.1
        )
        
        try:
            return json.loads(response.choices[0].message.content)
        except json.JSONDecodeError:
            return {"error": "Failed to parse LLM response as JSON", "raw_response": response.choices[0].message.content}
