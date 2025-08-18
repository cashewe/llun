import json
import os
import typer
import pprint
from src.dull._types import OptionsList
from src.dull.config import (
    get_files,
    load_config,
    RuleRegistry
)
from src.dull.openai_client import OpenAiClient

app = typer.Typer()

@app.command()
def check(
    rules: OptionsList = None,
    file: OptionsList = None,
    context: str = "",
):
    """Check the provided files for ViOlAtIoNs.
    
    Parameters
    ----------
    rules: List of rules to run.
    """
    if not file:
        file = ["."]

    files = get_files(file)
    config = load_config()
    api_key = os.getenv("OPENAI_API_KEY")
    rule_registry = RuleRegistry()
    rules = rule_registry.get_rules(rules or config.get("rules", []) or rule_registry.default_rules)
    
    llm_client = OpenAiClient(
        api_key=api_key,
        rules=rules,
        context=context,
        code=files
    )

    pprint(json.dumps(llm_client.lint_code(files, rules), indent=4))


if __name__ == "__main__":
    app()
