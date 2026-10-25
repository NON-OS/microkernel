#!/usr/bin/env python3
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
"""check_kernel_mirrors.py --self-test: the verdict on a made-up catalogue,
one good entry and each way a mirror can be missing."""

from check_kernel_mirrors import missing


def self_test():
    entries = [
        {"slug": "a", "kernel_mirror": "src/userspace/a", "feature": "nonos-capsule-a"},
        {"slug": "b", "kernel_mirror": "", "feature": "nonos-capsule-b"},
    ]
    dirs, rs, feats = {"src/userspace/a"}, {"src/userspace/a"}, {"nonos-capsule-a"}
    check = lambda d=dirs, r=rs, f=feats: missing(entries, d.__contains__, r.__contains__, f.__contains__)
    cases = [
        (check(), []),
        (check(d=set(), r=set()), [("a", "src/userspace/a does not exist")]),
        (check(r=set()), [("a", "src/userspace/a holds no .rs file")]),
        (check(f=set()), [("a", 'no kernel source names feature = "nonos-capsule-a"')]),
    ]
    failed = [i for i, (got, want) in enumerate(cases) if got != want]
    if failed:
        print(f"kernel mirrors: self-test failed on cases {failed}")
        return 1
    print(f"kernel mirrors: self-test passed, {len(cases) - 1} bad inputs caught and 1 good accepted")
    return 0
