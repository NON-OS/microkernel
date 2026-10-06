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

//! The release, as a mono label, from the repository's VERSION.

use alloc::format;
use alloc::string::String;

const VERSION: &str = include_str!("../../../../VERSION");

/// NØNOS and the release in capitals, such as NØNOS 0.9.2 BETA.
pub fn release() -> String {
    format!("N\u{d8}NOS {} BETA", VERSION.trim())
}
