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

//! The group keys one message delivered are wiped where they lie when the
//! holder goes: the GTK, the IGTK and the BIGTK.

use super::*;

impl Drop for GroupKeys {
    fn drop(&mut self) {
        wipe(&mut self.gtk);
        for k in [&mut self.igtk, &mut self.bigtk] {
            if let Some((key, _, _)) = k.as_mut() {
                wipe(key);
            }
        }
    }
}
