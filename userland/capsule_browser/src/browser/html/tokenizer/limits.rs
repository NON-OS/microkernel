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

/// Attributes kept on one tag. A tag carrying more is almost certainly
/// hostile; keeping the count small also keeps the duplicate check cheap.
/// The same number bounds what a script may add with `setAttribute`.
pub const MAX_TAG_ATTRS: usize = 64;

/// Bytes kept in one attribute value, above any real `data:` URI. A longer
/// value is dropped whole: half a URI or half a script is worse than none.
pub const MAX_VALUE: usize = 1 << 20;
