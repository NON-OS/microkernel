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

//! The service a typed tool runs: `tool.<handle>`, but `tool.linux.net` for
//! a `linux --box` whose box names a destination, which the kernel starts
//! in the personality's one terminal role that holds Network. The kernel
//! and the personality each check the box again (tool_capsules/net_box.rs,
//! boxed/terminal_run.rs), so a wrong choice here is refused, never run.

use alloc::vec::Vec;

pub fn service(handle: &[u8], args: &[&[u8]]) -> Vec<u8> {
    let net = matches!(args, [b"linux", b"--box", word, ..] if names_destinations(word));
    [&b"tool."[..], handle, if net { b".net" } else { b"" }].concat()
}

fn names_destinations(word: &[u8]) -> bool {
    word.strip_prefix(b"allow:")
        .is_some_and(|items| items.split(|c| *c == b',').any(|i| i.starts_with(b"net:")))
}
