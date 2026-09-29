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

/*
 * A hidden-only scan configuration admits only hidden files.
 *
 * The kernel's scan configuration is included by path. hidden_only set
 * include_hidden, which every fresh configuration already has, so
 * ScanConfig::new().hidden_only() still admitted every visible file. The
 * check below fails against that code.
 */

#[path = "../../../../src/fs/utils/types.rs"]
pub mod types;
mod tests;
