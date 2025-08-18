import json
import typer
from typing import Annotated
import pprint
from src.dull.openai_client import OpenAiClient
from src.dull.config import Config

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

    config = Config()
    
    llm_client = OpenAiClient(
        config=config,
        rules=rules,
        context=context,
        code=file,
    )

    pprint(json.dumps(llm_client.lint_code(file, rules), indent=4))


if __name__ == "__main__":
    app()
