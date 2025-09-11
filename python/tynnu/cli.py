import sys
from typing import List, Optional
from .tynnu import run_cli

def main(argv: Optional[List[str]] = None) -> int:
    """Main CLI entry point."""
    if argv is None:
        argv = sys.argv
    
    try:
        return run_cli(argv)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())