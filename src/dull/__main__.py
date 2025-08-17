# this file will be the cmd utility only
import typer

app = typer.Typer()

@app.command()
def dummy():
    pass


if __name__ == "__main__":
    app()
