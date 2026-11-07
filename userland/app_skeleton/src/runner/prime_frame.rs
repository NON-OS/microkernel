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

use crate::app::{App, AppManifest};
use crate::discover::Peers;
use crate::setup::WindowBinding;
use nonos_libc::mk_yield;
use nonos_toolkit::decorations::{DecorationHit, FrameState};

use super::boot::INITIAL_PAINT_ATTEMPTS;
use super::paint_frame::paint;

pub(super) fn prime_frame<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &mut WindowBinding,
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    for _ in 0..INITIAL_PAINT_ATTEMPTS {
        let state = FrameState { hover: DecorationHit::None, focused: true };
        if paint(app, manifest, binding, (state, false), peers, request_id) {
            return true;
        }
        mk_yield();
    }
    false
}
