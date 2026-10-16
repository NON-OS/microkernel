// NONOS Operating System (AGPL-3.0-or-later)
//! The per-boot address key reads zero after the drop.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use super::AddressKey;

#[test]
fn the_address_key_is_gone_after_it_is_dropped() {
    let mut slot = MaybeUninit::new(AddressKey::new(&[0x5A; 32]));
    let k = slot.as_mut_ptr();
    // SAFETY: `k` points at the AddressKey `slot` holds, which outlives the
    // reads and is not dropped again; the field read is a plain array.
    unsafe {
        assert_eq!(read_volatile(addr_of!((*k).key)), [0x5A; 32]);
        drop_in_place(k);
        assert_eq!(read_volatile(addr_of!((*k).key)), [0; 32]);
    }
}
