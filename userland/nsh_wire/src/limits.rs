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

//! The sizes every frame is held to, from docs/nsh/nsh-v1-wire.md.

/// The length field in front of every frame.
pub const LEN_BYTES: usize = 4;
/// The Poly1305 tag after every sealed frame.
pub const TAG_LEN: usize = 16;
/// The most terminal bytes one `INPUT` or `OUTPUT` carries.
pub const MAX_DATA: usize = 16_384;
/// The largest type byte and body of a hello.
pub const HELLO_CAP: usize = 2048;
use crate::auth_ok::MAX_AUTH_OK;
use crate::user_auth::MAX_USER_AUTH;

/// The largest type byte and body of `HOST_PROOF`, which carries the
/// attestation document.
pub const AUTH_CAP: usize = 1152 * 1024;
/// The largest type byte and body the server takes after `HOST_PROOF`: the
/// largest legal `USER_AUTH`. An `ABORT` is far smaller.
pub const USER_AUTH_CAP: usize = 1 + MAX_USER_AUTH;
/// The largest type byte and body the client takes after `USER_AUTH`: the
/// largest legal `AUTH_OK`. `AUTH_REFUSED` and `ABORT` are far smaller.
pub const AUTH_RESULT_CAP: usize = 1 + MAX_AUTH_OK;
/// The largest type byte and body once the session runs.
pub const SESSION_CAP: usize = 1 + MAX_DATA;
