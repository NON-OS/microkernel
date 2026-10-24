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
//! The takeover and what the driver says of itself: every enabled scanout
//! an output showing a zeroed surface of the host's size; CAPS, MODES and
//! STATE as the wire defines them for virtio-gpu.

use nonos_gfx_wire::body::caps::{HINTS, MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, PREFERRED};
use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_gfx_wire::{kind, ops, Caps, Modes, State};

use super::ask::ask;
use super::kit::{backed_live, shown};
use super::rig::{rig, rig_with};

#[test]
fn every_enabled_scanout_is_an_output_showing_a_zeroed_surface() {
    let mut r = rig_with(&[(1024, 768), (0, 0), (800, 600)]);
    for (s, n) in [(0, 1024 * 768), (2, 800 * 600)] {
        let px = shown(&r, s).expect("taken");
        assert!(px.len() == n && px.iter().all(|&p| p == 0), "scanout {s} zero, whole");
    }
    assert!(shown(&r, 1).is_none() && backed_live(&r));
    let caps = Caps::decode(&ask(&mut r, ops::CAPS, 0, &[]).1).unwrap();
    assert_eq!((caps.kind, caps.outputs), (kind::VIRTIO_GPU, 2));
    assert_eq!(caps.flags, MODE_SET | MODE_RANGE | HINTS);
    let st = State::decode(&ask(&mut r, ops::STATE, 1, &[]).1).unwrap();
    assert_eq!((st.width, st.height, st.hint_width, st.flags), (800, 600, 800, SCANNING_OUT));
}

#[test]
fn modes_lead_with_the_host_s_size_shown_now() {
    let mut r = rig();
    let m = Modes::decode(&ask(&mut r, ops::MODES, 0, &[]).1).unwrap();
    let all = m.as_slice();
    assert_eq!(all.len(), 1, "the host's size, also the one shown");
    assert_eq!((all[0].width, all[0].height), (640, 480));
    assert_eq!(all[0].flags, PREFERRED | FROM_DEVICE | CURRENT);
    assert!(ask(&mut r, ops::MODES, 1, &[]).0 < 0, "no second output");
}
