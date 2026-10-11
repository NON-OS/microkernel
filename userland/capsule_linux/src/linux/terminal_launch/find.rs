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

//! Where a terminal's program is looked for, and how a refusal is worded.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::launch::Launch;

/// The built-in BusyBox for `program` when the Linux tree has no such file.
/// It is part of this capsule's own image, measured by the manifest that
/// admitted the capsule, so it needs no proof from the store.
pub(super) fn built_in_for(program: &[u8], args: Vec<Vec<u8>>) -> Option<Launch> {
    let name = super::super::built_in::serves(program)?;
    let argv0 = (name != b"busybox").then(|| name.to_vec());
    Some(Launch { args, argv0, ..super::super::built_in::built_in() })
}

pub(super) fn text(path: &[u8]) -> String {
    String::from_utf8_lossy(path).into_owned()
}

/// The PATH a bare name was looked for along, as the guest's environment
/// gives it.
pub(super) fn searched() -> String {
    let env = crate::linux::env::default();
    let path = env.iter().find_map(|v| v.strip_prefix(b"PATH=")).unwrap_or(b"/bin");
    text(path)
}
