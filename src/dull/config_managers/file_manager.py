from pathlib import Path

from pydantic import BaseModel


class File(BaseModel):
    """Representation of a file."""
    name: str
    content: str

    def __str__(self) -> str:
        return f"#{self.name}\n{self.content}"


class Files:
    """Manages the file requests the user makes."""

    def __init__(
        self,
        paths: list[str] = ["."]
    ):
        self.files: list[File] = []
        resolved_paths = Files._get_paths(paths)
        self._get_files(resolved_paths)

    @staticmethod
    def _get_paths(file_patterns: list[str]) -> list[Path]:
        """Get the list of files the user has requested and validate they exist."""
        file_list = []

        for pattern in file_patterns:
            if pattern == ".":
                return list(Path(".").rglob("*.py"))  # by default, run exclusively on .py files to save tokens.
            
            elif Path(pattern).exists():
                file_list.append(Path(pattern))
            
            else:
                raise FileNotFoundError("You've gone and asked for a file you haven't even provided. gimp.")

        return file_list

    def _get_files(self, paths: list[Path]) -> None:
        for path in paths:
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()

            self.files.append(
                File(
                    name=str(path),
                    content=content,
                )
            )

    def __str__(self) -> str:
        return "\n".join([str(file) for file in self.files])
    