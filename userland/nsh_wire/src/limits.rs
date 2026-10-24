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

//! The sizes every frame is held to, from docs/design/nsh-v1-wire.md.

/// The length field in front of every frame.
pub const LEN_BYTES: usize = 4;
/// The Poly1305 tag after every sealed frame.
pub const TAG_LEN: usize = 16;
/// The most terminal bytes one `INPUT` or `OUTPUT` carries.
pub const MAX_DATA: usize = 16_384;
/// The largest type byte and body of a hello.
pub const HELLO_CAP: usize = 2048;
/// The largest type byte and body while the host and the user prove
/// themselves: `HOST_PROOF` carries the attestation document.
pub const AUTH_CAP: usize = 262_144;
/// The largest type byte and body once the session runs.
pub const SESSION_CAP: usize = 1 + MAX_DATA;
