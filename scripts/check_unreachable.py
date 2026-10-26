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
"""Exported kernel functions that nothing calls.

rustc's dead-code lint never fires on a `pub fn` in a library crate, and the
kernel is a library crate. So a security mechanism can be written, reviewed,
merged and shipped with no line of it ever running, which is what happened to
KASLR, to the stack guards, to the kernel W^X check and to the periodic
security scan above it. Each site is listed in
scripts/baselines/unreachable.txt, a list that may only shrink.

A name is reachable when some other file mentions it outside an import. A
`pub use` only carries the name up the module tree, on every line of it; it
is not a call. A name a `use` renames is called where its new name is, and a
crate outside src/ that mounts a kernel file by #[path] calls what it names.
"""

import sys
import tempfile
from pathlib import Path

import gate
from unreachable_scan import unreachable

BASELINE = Path("scripts/baselines/unreachable.txt")


# Each limit the scan once had, as a decoy beside the one it must still find.
DECOYS = {
    "src/decoy.rs": "pub fn never_called() {}\npub fn called() {}\npub fn called_on_aarch64() {}\n"
    "pub fn reexported_only() {}\n",
    "src/user.rs": "use crate::decoy::never_called;\nfn f() { called(); }\n",
    # Its only caller is aarch64's, which counts; its own export does not.
    "src/arch/aarch64/sync.rs": "fn g() { called_on_aarch64(); }\npub fn aarch64_only() {}\n",
    # A continuation line of a multi-line `pub use` is not a call.
    "src/exports.rs": "pub use crate::decoy::{\n    reexported_only,\n};\n",
    # A call through a renaming `use` calls the name it renames.
    "src/keys.rs": "pub fn brightness() {}\npub fn renamed_only() {}\n",
    "src/event.rs": "use crate::keys::{brightness as brightness_key, renamed_only as never};\n"
    "fn e() { brightness_key(); }\n",
    # A crate that mounts a kernel file calls what it names; one that does
    # not mount it calls nothing there.
    "src/wire.rs": "pub fn wire_parse() {}\npub fn wire_unused() {}\n",
    "userland/proofs/Cargo.toml": "[package]\nname = \"proofs\"\n",
    "userland/proofs/src/lib.rs": '#[path = "../../../src/wire.rs"]\nmod wire;\n'
    "fn t() { wire::wire_parse(); }\n",
    "userland/other/Cargo.toml": "[package]\nname = \"other\"\n",
    "userland/other/src/lib.rs": "fn u() { wire_unused(); }\n",
    # A mount of a directory brings the files under it.
    "src/sess/api.rs": "pub fn sess_login() {}\npub fn sess_unused() {}\n",
    "userland/whole/Cargo.toml": "[package]\nname = \"whole\"\n",
    "userland/whole/src/lib.rs": '#[path = "../../../src/sess"]\nmod sess {\n    pub mod api;\n}\n'
    "fn w() { sess::api::sess_login(); }\n",
}
WANT = ["src/decoy.rs:1 never_called", "src/decoy.rs:4 reexported_only",
        "src/keys.rs:2 renamed_only", "src/sess/api.rs:2 sess_unused", "src/wire.rs:2 wire_unused"]


def self_test():
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for rel, text in DECOYS.items():
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_text(text)
        found = unreachable(root)
        if found != WANT:
            print(f"unreachable: self-test failed, found {found}")
            return 1
    print("unreachable: self-test passed, the decoys were reported; an import, a re-export's "
          "continuation line and a crate that did not mount the file saved none; an aarch64 "
          "call, a renamed call and a mounting crate's call, through a file or a directory, did")
    return 0


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        return self_test()
    return gate.run("unreachable", "nothing calls", unreachable, BASELINE, args)


if __name__ == "__main__":
    sys.exit(main())
