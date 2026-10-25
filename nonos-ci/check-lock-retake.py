#!/usr/bin/env python3
# No lock in the kernel is taken again by a path that already holds it (cpu-01).
# A spin lock taken twice on one path, with interrupts masked, stops its CPU
# for good, and on one CPU the machine: surface_registry's GATE did, on the
# first exit, before F1. lock_retake/graph.py says exactly what is read; it
# runs on src/ of the checkout, or of `root` when one is given.
#   check-lock-retake.py [root]     every finding, and fails on any
#   check-lock-retake.py --self     the fixtures in lock_retake/fixtures/

import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from lock_retake.graph import findings  # noqa: E402
from lock_retake.text import strip  # noqa: E402


def scan(src: Path) -> list[str]:
    paths = sorted(src.rglob("*.rs"))
    return findings([(str(p.relative_to(src.parent)), strip(p.read_text())) for p in paths])


def self_test() -> int:
    """Each fixture says, in its `// expect:` lines, every finding it holds."""
    bad = 0
    cases = sorted((HERE / "lock_retake/fixtures").glob("*.rs"))
    for case in cases:
        text = case.read_text()
        want = sorted(l.split("expect:", 1)[1].strip() for l in text.splitlines() if "expect:" in l)
        got = sorted(f.split(": ", 1)[1] for f in findings([(case.name, strip(text))]))
        if got != want:
            print(f"check-lock-retake: {case.name}: wanted {want}, found {got}")
            bad += 1
    print(f"check-lock-retake: {len(cases)} fixtures, {len(cases) - bad} as written")
    return 1 if bad or not cases else 0


def main() -> int:
    if sys.argv[1:] == ["--self"]:
        return self_test()
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE.parent
    found = scan(root / "src")
    for line in found:
        print(f"check-lock-retake: {line}")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
