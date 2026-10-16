// NONOS Operating System (AGPL-3.0-or-later)
//! A join's pairwise and group keys read zero once its outcome goes, on
//! every path: installed, or refused by the CAM.

use core::mem::MaybeUninit;
use core::ptr::{drop_in_place, read_volatile};

use nonos_wifi_core::key::HeldKey;
use nonos_wifi_core::wpa::akm::Akm;

use crate::assoc::Outcome;

#[test]
fn a_joins_keys_read_zero_once_its_outcome_is_dropped() {
    let joined = Outcome::Joined {
        bssid: [2, 0, 0, 0, 0, 1],
        channel: 6,
        ptk: HeldKey([0x11; 16]),
        gtk: HeldKey([0x22; 16]),
        gtk_id: 1,
        akm: Akm::Psk,
        supplicant: None,
    };
    let mut slot = MaybeUninit::new(joined);
    let o = slot.as_mut_ptr();
    // SAFETY: `o` points at the Outcome `slot` holds, which outlives the reads
    // and is not dropped again; after the drop its bytes are still a valid
    // Outcome (no supplicant, keys of zeros), so the reference is sound.
    unsafe {
        drop_in_place(o);
        let Outcome::Joined { ptk, gtk, .. } = &*o else { panic!("joined") };
        assert_eq!(read_volatile(&ptk.0), [0; 16], "the pairwise key");
        assert_eq!(read_volatile(&gtk.0), [0; 16], "the group key");
    }
}
