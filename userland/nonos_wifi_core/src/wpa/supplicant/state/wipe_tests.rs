// NONOS Operating System (AGPL-3.0-or-later)
//! The supplicant's PMK, SNonce, PTK, GTK and IGTK read zero after the drop.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use super::fields::Supplicant;
use super::kinds::Config;
use crate::wpa::akm::Akm;

#[test]
fn the_supplicants_keys_read_zero_after_it_is_dropped() {
    let rsne = [0x30, 0x14, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 2];
    let s = Supplicant::configure(&Config {
        pmk: [0x44; 32],
        aa: [2, 0, 0, 0, 1, 0],
        spa: [2, 0, 0, 0, 0, 0],
        snonce: [0x55; 32],
        akm: Akm::Psk,
        own_rsne: &rsne,
        own_rsnxe: None,
        ap_rsne: &rsne,
        ap_rsnxe: None,
        pmf: false,
    });
    let mut slot = MaybeUninit::new(s);
    let p = slot.as_mut_ptr();
    // SAFETY: `p` points at the Supplicant `slot` holds, which outlives the
    // reads and is not dropped again; only its key arrays are read.
    unsafe {
        ((*p).ptk, (*p).gtk, (*p).igtk) = ([0x66; 48], [0x77; 32], [0x88; 16]);
        drop_in_place(p);
        assert_eq!(read_volatile(addr_of!((*p).pmk)), [0; 32], "the PMK");
        assert_eq!(read_volatile(addr_of!((*p).snonce)), [0; 32], "the SNonce");
        assert_eq!(read_volatile(addr_of!((*p).ptk)), [0; 48], "the PTK");
        assert_eq!(read_volatile(addr_of!((*p).gtk)), [0; 32], "the GTK");
        assert_eq!(read_volatile(addr_of!((*p).igtk)), [0; 16], "the IGTK");
    }
}
