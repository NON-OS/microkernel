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

use crate::browser::url::{join, Url};

use super::super::display_list::BoxDocument;

impl BoxDocument {
    /// Whether the image fetched as `key` (an absolute URL) was laid out
    /// before its natural size was known; `base` resolves the page's
    /// relative sources the way the fetch did.
    pub fn awaits(&self, key: &str, base: Option<&Url>) -> bool {
        self.unsized_imgs.iter().any(|src| match base {
            Some(b) => join(b, src) == key,
            None => src == key,
        })
    }
}
