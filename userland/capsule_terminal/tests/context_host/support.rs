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

//! Renders a context line for the fixed user.

use crate::term::context::context_line;
use crate::term::identity::USER_FALLBACK as USER;

pub fn line(host: &[u8], cwd: &[u8], home: &[u8]) -> String {
    let mut out = [0u8; 96];
    let n = context_line(USER, host, cwd, home, b"ls", &mut out);
    String::from_utf8(out[..n].to_vec()).unwrap()
}
