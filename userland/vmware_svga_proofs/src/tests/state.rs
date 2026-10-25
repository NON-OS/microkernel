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

//! What STATE does after a guest reset.

use super::ask::ask;
use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::{ops, State};

#[test]
fn state_after_a_guest_reset_sets_the_mode_again_and_says_lost() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let st = State::decode(&ask(&mut d.server, ops::STATE, &[]).1).unwrap();
    assert_eq!(st.flags & (LOST | SCANNING_OUT), SCANNING_OUT, "nothing lost yet");
    with_svga(|s| (s.enable, s.width, s.height, s.bpp) = (0, 640, 480, 8));
    let lost = State::decode(&ask(&mut d.server, ops::STATE, &[]).1).unwrap();
    assert_eq!(lost.flags & (LOST | SCANNING_OUT), LOST | SCANNING_OUT);
    assert_eq!(lost.generation, st.generation + 1, "the compositor repaints all of it");
    assert_eq!(with_svga(|s| (s.enable, s.width, s.height, s.bpp)), (1, 1024, 768, 32));
}
