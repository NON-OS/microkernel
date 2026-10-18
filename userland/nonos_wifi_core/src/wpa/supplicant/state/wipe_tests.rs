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

//! The supplicant's PMK, nonces, PTK, GTK, IGTK and rekey seed read zero
//! after the drop.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use super::{Config, Supplicant};
use crate::wpa::akm::Akm;

#[test]
fn the_supplicants_keys_read_zero_after_it_is_dropped() {
    let rsne = [0x30, 0x14, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 2];
    let mut s = Supplicant::configure(&Config {
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
    s.reseed_rekey([0x99; 32]);
    let mut slot = MaybeUninit::new(s);
    let p = slot.as_mut_ptr();
    // SAFETY: `p` points at the Supplicant `slot` holds, which outlives the
    // reads and is not dropped again; only its key arrays are read.
    unsafe {
        ((*p).ptk, (*p).gtk, (*p).igtk) = ([0x66; 64], [0x77; 32], [0x88; 16]);
        (*p).anonce = [0x99; 32];
        drop_in_place(p);
        assert_eq!(read_volatile(addr_of!((*p).pmk)), [0; 32], "the PMK");
        assert_eq!(read_volatile(addr_of!((*p).snonce)), [0; 32], "the SNonce");
        assert_eq!(read_volatile(addr_of!((*p).ptk)), [0; 64], "the PTK");
        assert_eq!(read_volatile(addr_of!((*p).anonce)), [0; 32], "the ANonce");
        assert_eq!(read_volatile(addr_of!((*p).gtk)), [0; 32], "the GTK");
        assert_eq!(read_volatile(addr_of!((*p).igtk)), [0; 16], "the IGTK");
        assert_eq!(read_volatile(addr_of!((*p).rekey_seed)), [0; 32], "the rekey seed");
    }
}
