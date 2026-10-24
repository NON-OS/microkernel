// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The help text. The flags and exit codes it names are the interface; what
//! each check covers is docs/release/nonos-harden.md.

pub const SHORT: &str = "\
usage: nonos-harden check --release DIR [--tree DIR] [--format text|json] [--out FILE]
       nonos-harden --help | --version
";

pub const LONG: &str = "\
nonos-harden: one GO or NO-GO before an image touches a machine

usage:
  nonos-harden check --release DIR [--tree DIR] [--format text|json] [--out FILE]
  nonos-harden --help | --version

`check` runs every verifier this tree holds over the sealed release in DIR
(the directory the seal writes: nonos-release.json beside what it sealed)
and prints one verdict. GO means every check ran and passed. A check that
fails, or that could not run, makes the verdict NO-GO, and the verdict
names it and says why. A development image that passes reads
  GO (development image, not a release)

  --release DIR   the sealed release to check (required)
  --tree DIR      the checkout that built it (default: the one this command
                  is run inside)
  --format F      text (the default) or json (format nonos-harden-verdict/1)
  --out FILE      also write the verdict to FILE

exit status:
  0 GO    1 NO-GO    2 bad command line    3 the verdict could not be written

Every check, what it covers, and what GO does not mean:
  docs/release/nonos-harden.md
";
