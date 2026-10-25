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

"""nonos-linux-pack [--sysroot ...] packs a program; `syscalls` measures one;
`policy` writes a market release's guest_policy."""

import sys

from . import pack, policy_cli, syscalls


def main() -> int:
    argv = sys.argv[1:]
    if argv[:1] == ["syscalls"]:
        return syscalls.main(argv[1:])
    if argv[:1] == ["policy"]:
        return policy_cli.main(argv[1:])
    return pack.main(argv)


if __name__ == "__main__":
    sys.exit(main())
