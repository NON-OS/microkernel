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

//! The register window as the broker mapped it, which an MSI-X table in the
//! BAR can end before the BAR does: nothing past it is touched.

use nonos_devmodel::FakeBar;

use crate::constants::VS_EM4L;
use crate::discover::{Found, Survey};
use crate::error::HdaError;
use crate::proofs::lifecycle::board::DEVICE;
use crate::setup::run;

/// Apollo Lake, whose DMA latency register VS_EM4L lies past the first page.
const APL: u16 = 0x5a98;
/// Sunrise Point (Skylake), whose link clock is moved through the multi-link
/// capability; the test places that past the first page, its LCTL at +0x44.
const SKL: u16 = 0xa170;
const ML: usize = 0x1100;
const LCTL: usize = ML + 0x44;

/// One attempt on Intel's `device`, a 16 KiB BAR mapped only to the end of
/// its first page. The page past that is host memory here, laid out by
/// `place`, so a touch shows instead of faulting. The window reads all
/// zeros, so the attempt ends on VMAJ, after the vendor registers.
fn cut_short(device: u16, place: impl FnOnce(&FakeBar)) -> FakeBar {
    let bar = FakeBar::new(0x2000);
    place(&bar);
    let rings = [FakeBar::new(4096), FakeBar::new(4096)];
    let dma = rings.iter().zip([0x1000_0000, 0x1010_0000]).map(|(r, at)| (r.base(), at, 4096));
    nonos_libc::offer_dma(dma.collect());
    nonos_libc::offer_window(bar.base(), 0x1000);
    let (device_id, vendor, bar_size) = (DEVICE, 0x8086, 0x4000);
    let found =
        Found { device_id, vendor, device, subclass: 3, irq_pin: 1, irq_line: 0xff, bar_size };
    let survey = Survey { hda: [Some(found), None, None, None], amd_acp: false, intel_sst: None };
    assert_eq!(run(&survey, &mut None).err(), Some(HdaError::UnsupportedController));
    bar
}

#[test]
fn a_register_past_the_mapped_window_is_never_touched() {
    let bar = cut_short(APL, |b| b.present32(VS_EM4L as usize, 0xffff_ffff));
    assert_eq!(bar.wrote32(VS_EM4L as usize), 0xffff_ffff, "VS_EM4L, past the window, was written");
}

#[test]
fn a_multi_link_capability_past_the_mapped_window_is_never_touched() {
    // LLCH names the capability; its link is on 6 MHz with SPA == CPA, which
    // the clock move would take down and up again.
    let bar = cut_short(SKL, |b| {
        b.present16(0x14, ML as u16);
        b.present32(ML, 0x2 << 16);
        b.present32(LCTL, 0);
    });
    assert_eq!(bar.wrote32(LCTL), 0, "LCTL, past the window, was written");
}
