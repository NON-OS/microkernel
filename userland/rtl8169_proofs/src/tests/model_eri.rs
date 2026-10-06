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

//! A part that serves the ERI window (ERIDR 0x70, ERIAR 0x74) as the
//! datasheet describes: a command with FLAG set is a write the part takes
//! and then clears FLAG; one with FLAG clear is a read the part answers by
//! filling ERIDR and setting FLAG. Once done the part shows ERIAR as 0 (a
//! write) or bare FLAG (a read), the only bit the driver looks at, so a
//! command the driver writes next is always a change the model can see.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nonos_devmodel::FakeBar;

const ERIDR: usize = 0x70;
const ERIAR: usize = 0x74;
const FLAG: u32 = 1 << 31;

/// Every write the part took, as (address, byte-enable bits, data).
pub type Taken = Arc<Mutex<Vec<(u32, u32, u32)>>>;

/// The window is bytes, so a dword read here can straddle a driver store;
/// read until two looks agree.
fn settled(bar: &FakeBar, offset: usize) -> u32 {
    let mut seen = bar.wrote32(offset);
    loop {
        let again = bar.wrote32(offset);
        if again == seen {
            return seen;
        }
        seen = again;
    }
}

pub fn eri_part(start: &[(u32, u32)], taken: Taken) -> impl Fn(&FakeBar) + Send + 'static {
    let store: Mutex<HashMap<u32, u32>> = Mutex::new(start.iter().copied().collect());
    move |bar| {
        let cmd = settled(bar, ERIAR);
        if cmd == 0 || cmd == FLAG {
            return;
        }
        let addr = cmd & 0xFFF;
        let mut store = store.lock().unwrap_or_else(|p| p.into_inner());
        if cmd & FLAG != 0 {
            let data = settled(bar, ERIDR);
            taken.lock().unwrap_or_else(|p| p.into_inner()).push((addr, cmd & 0xF000, data));
            store.insert(addr, data);
            bar.present32(ERIAR, 0);
        } else {
            bar.present32(ERIDR, store.get(&addr).copied().unwrap_or(0));
            bar.present32(ERIAR, FLAG);
        }
    }
}
