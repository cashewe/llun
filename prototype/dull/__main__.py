import json
import typer
from typing import Annotated
import pprint
from dull.openai_client import OpenAiClient
from dull.config import ConfigFactory

OptionsList = Annotated[
    list[str] | None, typer.Option()
]  # this will apparently allow you to input multiple values
# its not super clear tbh the docs are ambiguous when it comes to special types...

app = typer.Typer()


@app.command()
def check(
    rules: OptionsList = None,
    files: OptionsList = None,
    context: str = "",
):
    """Check the provided files for ViOlAtIoNs.

    Parameters
    ----------
    rules: List of rules to run.
    """
    config = ConfigFactory().create_config(
        rules=rules,
        files=files,
    )

    llm_client = OpenAiClient(
        config=config,
        context=context,
    )

    linted_code = llm_client.lint_code()

    pprint.pprint(json.dumps(linted_code, indent=4))


if __name__ == "__main__":
    app()
