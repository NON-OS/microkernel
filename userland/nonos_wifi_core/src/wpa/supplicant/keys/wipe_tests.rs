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

//! A message's group keys read zero once the holder is dropped.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use super::GroupKeys;

#[test]
fn a_messages_group_keys_are_gone_after_it_is_dropped() {
    let keys = GroupKeys {
        gtk: [0x11; 32],
        gtk_id: 1,
        igtk: Some(([0x22; 16], 4, [0; 6])),
        bigtk: Some(([0x33; 16], 6, [0; 6])),
        rsc: 0,
    };
    let mut slot = MaybeUninit::new(keys);
    let k = slot.as_mut_ptr();
    // SAFETY: `k` points at the GroupKeys `slot` holds, which outlives the
    // reads and is not dropped again; the fields read are plain values.
    unsafe {
        drop_in_place(k);
        assert_eq!(read_volatile(addr_of!((*k).gtk)), [0; 32], "the GTK");
        let igtk = read_volatile(addr_of!((*k).igtk));
        assert_eq!(igtk.map(|(key, _, _)| key), Some([0; 16]), "the IGTK");
        let bigtk = read_volatile(addr_of!((*k).bigtk));
        assert_eq!(bigtk.map(|(key, _, _)| key), Some([0; 16]), "the BIGTK");
    }
}
