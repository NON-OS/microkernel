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

use alloc::vec::Vec;

use crate::command::output::Output;

/// The release this terminal was built in, from the repository's VERSION.
const VERSION: &str = include_str!("../../../../../VERSION");

/*
 * It said "NONOS terminal v0.1" in a 0.9 release, named a namespace no
 * manifest carries ("terminal0") and stated the signature scheme as a fixed
 * line. The release comes from VERSION, the namespace from Capsule.mk, and
 * the signer from the kernel's registry entry for this process.
 */
pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    let mut head = Vec::from(&b"NONOS "[..]);
    head.extend_from_slice(VERSION.trim_end().as_bytes());
    head.extend_from_slice(b" terminal");
    out.writeln(&head);
    out.writeln(b"capsule namespace: systems.nonos.app.terminal");
    out.writeln(b"abi: nonos-sys-v1 (tag4 dispatch)");
    let mut line = Vec::from(&b"signed by: "[..]);
    line.extend_from_slice(super::receipt::own_line());
    out.writeln(&line);
}
