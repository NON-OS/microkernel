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

//! A package may not ship a program's proof, nor the policy the install role
//! vouches beside it (plan/30: place_entry refuses both names).

use crate::install::proof_names::is_proof_name;

#[test]
fn every_proof_and_policy_name_is_refused_wherever_it_sits() {
    for name in [
        &b"usr/bin/jq.zk_trailer.bin"[..],
        b"usr/bin/jq.manifest.bin",
        b"usr/bin/jq.nonos_id_cert.bin",
        b"usr/bin/jq.nonos_linux_policy.bin",
        b"usr/bin/jq.nonos_linux_policy.zk_trailer.bin",
        b"deep/dir/x.nonos_linux_policy.bin",
    ] {
        assert!(is_proof_name(name), "{}", String::from_utf8_lossy(name));
    }
}

#[test]
fn ordinary_files_land() {
    for name in [&b"usr/bin/jq"[..], b"usr/share/doc/policy.bin", b"etc/manifest", b"a.bin"] {
        assert!(!is_proof_name(name), "{}", String::from_utf8_lossy(name));
    }
}
