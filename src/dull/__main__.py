import typer
from src.dull.helpers._types import OptionsList

app = typer.Typer()

@app.command()
def check(
    rules: OptionsList = None,
):
    """Check the provided files for ViOlAtIoNs.
    
    Parameters
    ----------
    rules: List of rules to run.
    """
    for rule in rules:
        print(f"you've selected the rule {rule}, but i cant be assed to implement that yet.")


if __name__ == "__main__":
    app()
