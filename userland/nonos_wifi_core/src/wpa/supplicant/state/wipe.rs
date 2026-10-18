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

//! A supplicant holds the PMK, every key the handshakes gave and the seed
//! of its rekeys' SNonces. It is wiped when dropped, so a session's end, a failed join and every clone a
//! driver takes all leave nothing behind.

use super::*;
use crate::vault::wipe;

impl Drop for Supplicant {
    fn drop(&mut self) {
        for k in [&mut self.pmk[..], &mut self.snonce, &mut self.anonce, &mut self.ptk] {
            wipe(k);
        }
        wipe(&mut self.gtk);
        wipe(&mut self.igtk);
        wipe(&mut self.rekey_seed);
        if let Some((key, _, _)) = self.bigtk.as_mut() {
            wipe(key);
        }
    }
}
