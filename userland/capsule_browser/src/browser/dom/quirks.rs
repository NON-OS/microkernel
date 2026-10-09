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

/// The document's mode, which its doctype selects (13.2.6.4.1). Parsing
/// consults it once, for whether a table closes an open paragraph; selector
/// matching and layout have rules of their own that depend on it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Quirks {
    /// "no-quirks mode": a standards doctype, or a fragment.
    #[default]
    No,
    /// "limited-quirks mode": the transitional and frameset doctypes.
    Limited,
    /// "quirks mode": no doctype, or a legacy one.
    Full,
}
