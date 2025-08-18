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
    config = Config()

    # check for command line use of rules or files, and replace them in the Config if discovered
    # should this be handled by the config or should we have a configmanager that does the alterations?
    # probs the latter but its sweatty af you have to admit...
    
    llm_client = OpenAiClient(
        config=config,
        context=context,
    )

    pprint(json.dumps(llm_client.lint_code(), indent=4))


if __name__ == "__main__":
    app()
