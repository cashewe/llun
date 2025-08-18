import json
from pathlib import Path
import sys
from typing import Any

import openai

from _types import Rule
from data.system_prompt import prompt, formatted_schema


class OpenAiClient:
    """Handles communication with the LLM service."""
    
    def __init__(
        self,
        api_key: str,
        rules=str,
        context=str,
        code=str,
    ):
        self.client = openai.OpenAI(api_key=api_key)
        self.system_prompt = prompt.format(
            rules=rules,
            context=context,
            code=code,
            formatted_schema=formatted_schema,
        )

    def lint_code(self, files: list[Path], rules: list[Rule]) -> dict[str, Any]:
        """Send code and rules to LLM for analysis."""
        formatted_rules = [rule.to_dict() for rule in rules]
        
        # Read file contents
        file_contents = {}
        for file_path in files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    file_contents[str(file_path)] = f.read()
            except Exception as e:
                print(f"Warning: Could not read {file_path}: {e}", file=sys.stderr)
        
        user_message = {
            "rules_to_check": formatted_rules,
            "files": file_contents,
            "expected_response_schema": formatted_schema,
            "instructions": "Analyze the provided files against the given rules. Return violations in the specified JSON schema format."
        }
        
        response = self.client.chat.completions.create(
            model="gpt-4",
            messages=[
                {"role": "system", "content": self.system_prompt},
                {"role": "user", "content": json.dumps(user_message, indent=2)}
            ],
            temperature=0.1
        )
        
        try:
            return json.loads(response.choices[0].message.content)
        except json.JSONDecodeError:
            return {"error": "Failed to parse LLM response as JSON", "raw_response": response.choices[0].message.content}
