// NONOS Operating System (AGPL-3.0-or-later)
//! A message's group keys read zero once the holder is dropped.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use super::store::GroupKeys;

#[test]
fn a_messages_group_keys_are_gone_after_it_is_dropped() {
    let keys =
        GroupKeys { gtk: [0x11; 16], gtk_id: 1, igtk: Some(([0x22; 16], 4, [0; 6])), rsc: 0 };
    let mut slot = MaybeUninit::new(keys);
    let k = slot.as_mut_ptr();
    // SAFETY: `k` points at the GroupKeys `slot` holds, which outlives the
    // reads and is not dropped again; the fields read are plain values.
    unsafe {
        drop_in_place(k);
        assert_eq!(read_volatile(addr_of!((*k).gtk)), [0; 16], "the GTK");
        let igtk = read_volatile(addr_of!((*k).igtk));
        assert_eq!(igtk.map(|(key, _, _)| key), Some([0; 16]), "the IGTK");
    }
}
