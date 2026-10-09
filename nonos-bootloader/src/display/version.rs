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

//! The release this loader belongs to, read from the repository's VERSION
//! at build time, so the loader and the kernel never name different ones.

const RAW: &str = include_str!("../../../VERSION");

/// The release channel; the kernel's build_info.rs names the same one.
pub const CHANNEL: &[u8] = b"beta";

/// The bare version, such as 0.9.2.
pub fn version() -> &'static [u8] {
    RAW.trim().as_bytes()
}

/// The release as a mono label: NØNOS 0.9.2 BETA.
pub fn version_label() -> crate::display::text::Text {
    let t = crate::display::text::Text::new().push(b"N\xD8NOS ").push(version()).push(b" ");
    CHANNEL.iter().fold(t, |t, &b| t.push(&[b.to_ascii_uppercase()]))
}
