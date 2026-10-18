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

//! Capabilities for the control proofs: a pacer whose string was read
//! already, for the transaction proofs, and the polls that read pipe B's
//! monitor's on the two-output rig.

use crate::ddc::caps::Caps;
use crate::ddc::caps_fetch::CapsFetch;
use crate::ddc::pace::Pacer;
use crate::tests::wire_rig::Rig;

/// A capabilities string naming the four controls, two inputs and on, off.
pub const CAPS: &[u8] = b"(prot(monitor)vcp(10 12 60(0F 11) D6(01 04)))";

/// A pacer for a display whose capabilities string was read already.
pub fn known() -> Pacer {
    let mut p = Pacer::new();
    p.caps = CapsFetch::known(Caps::parse(CAPS).unwrap());
    p
}

/// The serve loop's polls until pipe B's monitor has said what it has.
pub fn read_caps(d: &mut Rig) {
    for _ in 0..400 {
        d.poll_caps();
        d.clock.now.set(d.clock.now.get() + 60_000);
    }
    assert!(d.ddc[1].caps.caps().is_ok(), "the capabilities string was read");
}
