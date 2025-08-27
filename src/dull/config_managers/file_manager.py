from pathlib import Path

from pydantic import BaseModel


class File(BaseModel):
    """Representation of a file."""

    name: str
    content: str

    def __str__(self) -> str:
        return f"#{self.name}\n{self.content}"


class Files(BaseModel):
    """Manages the file requests the user makes."""

    paths: list[str] = ["."]
    files: list[File] = []

    def model_post_init(self, __context):
        resolved_paths = self._resolve_paths()
        self._get_files(resolved_paths)

    def _resolve_paths(self) -> list[Path]:
        """Get the list of files the user has requested and validate they exist."""
        resolved_paths = []

        for pattern in self.paths:
            if pattern == ".":
                resolved_paths.extend(
                    Path(".").rglob("*.py")
                )  # by default, run exclusively on .py files to save tokens.
                return resolved_paths

            elif Path(pattern).exists():
                resolved_paths.append(Path(pattern))

            else:
                raise FileNotFoundError(
                    "You've gone and asked for a file you haven't even provided. gimp."
                )

        return resolved_paths

    def _get_files(self, resolved_paths: list[Path]) -> None:
        for path in resolved_paths:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()

            self.files.append(
                File(
                    name=str(path),
                    content=content,
                )
            )

    def __str__(self) -> str:
        return "\n".join([str(file) for file in self.files])
