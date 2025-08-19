import json


class PromptManager:
    def __init__(self):
        self.system_prompt = PromptManager._load_system_prompt()
        self.user_prompt = PromptManager._load_user_prompt()

    @staticmethod
    def _load_system_prompt() -> str:
        with open('data/schema.json', 'r') as f:
            schema = json.load(f)
            
        formatted_schema = json.dumps(schema, indent=2)

        with open('data/system_prompt.txt', 'r', encoding='utf-8') as file:
            return file.read().format(formatted_schema)

    @staticmethod
    def _load_user_prompt() -> str:
        """NOTE: the user_prompt object needs to be formatted during use."""
        with open('data/user_prompt.txt', 'r', encoding='utf-8') as file:
            return file.read()
