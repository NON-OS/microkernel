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

use super::paste::clipboard_paste;
use crate::input::text::{first_line, PasteLine};

/// Read the clipboard's text into `out` and take its first line, for a
/// one-line field. Err when the clipboard cannot be reached; Ok(None) when
/// what it holds is not text. An empty clipboard is an empty line.
pub fn clipboard_paste_line(out: &mut [u8]) -> Result<Option<PasteLine<'_>>, &'static str> {
    let n = clipboard_paste(out)?;
    Ok(first_line(&out[..n]))
}
