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

//! Output 0's EDID from the device, through to the panel the wire offers:
//! any EDID a monitor might send (timings of every size in the base block
//! and in up to seven CTA-861 extensions, bad checksums, noise) and the
//! panel holds only modes the driver can make, each once, never more than the wire's
//! list, and a preferred mode only when it is one of those.

use super::device::{case, Reply};
use crate::setup::edid::panel;
use crate::tests::part::answers::header;
use crate::tests::wire_sweep::rng::Rng;

use super::edid_blob::blob;

/// Whether the driver can make a `w` x `h` primary: sides 640x480 to 4096,
/// its frame within one 32 MiB display grant at four bytes a pixel.
fn makes(w: u32, h: u32) -> bool {
    let sides = (640..=4096).contains(&w) && (480..=4096).contains(&h);
    sides && u64::from(w) * u64::from(h) * 4 <= 32 << 20
}

#[test]
fn any_monitor_s_edid_gives_a_panel_of_modes_the_driver_can_make() {
    let (mut r, mut offered, mut full) = (Rng(0x7061_6e01), 0, 0);
    for i in 0..2_000 {
        let edid = blob(&mut r);
        let mut bytes = header(0x1104, 0, 0);
        bytes.extend_from_slice(&(edid.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&edid);
        let (p, _, live) = case(Reply { bytes, claimed: None }, |q| panel(q, true));
        assert!(live, "case {i}");
        assert!(p.len <= p.modes.len(), "case {i}");
        full += (p.len == p.modes.len()) as u32;
        let modes = &p.modes[..p.len];
        for (k, m) in modes.iter().enumerate() {
            assert!(makes(m.width, m.height), "case {i}: {}x{} offered", m.width, m.height);
            let again = modes[..k].iter().any(|o| (o.width, o.height) == (m.width, m.height));
            assert!(!again, "case {i}: {}x{} offered twice", m.width, m.height);
        }
        if let Some((w, h)) = p.preferred {
            assert!(makes(w, h), "case {i}: preferred {w}x{h}");
        }
        offered += (p.len > 0) as u32;
    }
    assert!(offered >= 100, "only {offered} of 2000 panels offered a mode");
    assert!(full >= 20, "only {full} of 2000 panels filled the list: its bound is untried");
}
