// NONOS Operating System (AGPL-3.0-or-later)

//! A key on its way to the device reads zero once its holder is dropped, as
//! when an `Option` holding it is set to `None`.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use nonos_wifi_core::key::HeldKey;

#[test]
fn a_held_key_reads_zero_after_it_is_dropped() {
    let mut slot = MaybeUninit::new(HeldKey([0x77; 16]));
    let k = slot.as_mut_ptr();
    // SAFETY: `k` points at the HeldKey `slot` holds, which outlives the
    // reads and is not dropped again; the field read is a plain array.
    unsafe {
        assert_eq!(read_volatile(addr_of!((*k).0)), [0x77; 16]);
        drop_in_place(k);
        assert_eq!(read_volatile(addr_of!((*k).0)), [0; 16]);
    }
}
