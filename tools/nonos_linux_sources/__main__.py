# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""nonos-linux-sources [--out DIR]: every source recipe's content at DIR
(target/linux-sources), held to its pin, from the cache, the content
branch or upstream (__init__.py)."""

import argparse
import sys
from pathlib import Path

from .ensure import ensure
from .pins import ROOT


def main(argv) -> int:
    ap = argparse.ArgumentParser(prog="nonos-linux-sources", description=__doc__.splitlines()[0])
    ap.add_argument("--out", type=Path, default=ROOT / "target/linux-sources")
    ap.add_argument("--repo", type=Path, default=ROOT, help="the checkout whose origin holds the content branch")
    a = ap.parse_args(argv)
    try:
        for package, came in ensure(a.out.resolve(), repo=a.repo.resolve()):
            print(f"  linux-sources: {package}: {came}")
    except ValueError as why:
        print(f"nonos-linux-sources: {why}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
