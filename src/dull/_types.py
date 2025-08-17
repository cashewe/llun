from dataclasses import dataclass
from typing import Annotated

import typer

OptionsList = Annotated[list[str] | None, typer.Option()]  # this will apparently allow you to input multiple values 
# its not super clear tbh the docs are ambiguous when it comes to special types...

@dataclass
class Rule:
    """An architectural rule to be enforced."""
    brief_description: str
    long_description: str
    example: str
