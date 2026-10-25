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
usage: nonos-harden check --release DIR [--tree DIR] [--format text|json]
                          [--out FILE [--sign-key KEY]]
       nonos-harden verify-signature --verdict FILE --signature FILE --public-key KEY
       nonos-harden --help | --version
";

pub const LONG: &str = "\
nonos-harden: one GO or NO-GO before an image touches a machine

usage:
  nonos-harden check --release DIR [--tree DIR] [--format text|json]
                     [--out FILE [--sign-key KEY]]
  nonos-harden verify-signature --verdict FILE --signature FILE --public-key KEY
  nonos-harden --help | --version

`check` runs every verifier this tree holds over the sealed release in DIR
(the directory the seal writes: nonos-release.json beside what it sealed)
and prints one verdict. GO means every check ran and passed. A check that
fails, or that could not run, makes the verdict NO-GO, and the verdict
names it and says why. A development image that passes reads
  GO (development image, not a release)
A GO whose checks carry limits counts them in its line (\"GO, 2 declared
limits\") and lists each under it. A verifier's gap is a limit only if the
tree declares it in scripts/baselines/declared-limits.txt; any other is NO-GO.

  --release DIR   the sealed release to check (required)
  --tree DIR      the checkout that built it (default: the one this command
                  is run inside)
  --format F      text (the default) or json (format nonos-harden-verdict/1)
  --out FILE      also write the verdict to FILE
  --sign-key KEY  sign FILE with your own Ed25519 key (its 32-byte seed, 64
                  hex digits, or a nonos-sign seed file), into FILE.sig;
                  a NONOS key is refused, before any check runs

`verify-signature` checks that FILE.sig is KEY's signature of FILE, byte for
byte, and that FILE names KEY (64 hex digits, or a public key file) as its
signer. It does not say whether the verdict is GO; the verdict does.

exit status:
  0 GO, a release (or: the signature verifies)    1 NO-GO    2 bad command
  line, a key refused, or a tree inside the release    3 the verdict could
  not be written or signed    4 the signature does not verify    5 GO, but a
  development image: never a release, whatever a script does with the status

Every check, what it covers, and what GO does not mean:
  docs/release/nonos-harden.md
";
