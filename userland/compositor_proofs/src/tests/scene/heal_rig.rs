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

//! The idle heal as the server loop runs it, and an empty desktop.

use super::attach_kernel::FakeKernel;
use super::frame_model::Screen;
use crate::attach::AttachCache;
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;
use crate::state::visible_drop::missed;

pub(super) fn heal(scene: &SceneTable, acc: &mut DamageAccumulator) {
    missed(scene).for_each(|r| acc.accumulate(r));
}

pub(super) fn rig() -> (Screen, SceneTable, AttachCache, FakeKernel, DamageAccumulator) {
    let scene = SceneTable::new();
    (
        Screen::composed(&scene),
        scene,
        AttachCache::new(),
        FakeKernel::default(),
        DamageAccumulator::new(),
    )
}
