import json
import os
import typer
from typing import Annotated
import pprint
from src.dull.config import RuleRegistry
from src.dull.openai_client import OpenAiClient

OptionsList = Annotated[list[str] | None, typer.Option()]  # this will apparently allow you to input multiple values 
# its not super clear tbh the docs are ambiguous when it comes to special types...

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
