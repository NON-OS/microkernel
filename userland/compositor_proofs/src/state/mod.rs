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

//! The compositor's `state` as it ships: the attach cache, damage, the
//! scene and the steps that change it, the indicators and the trusted band.
//! Each file's `super::` resolves to its neighbours here, as in-tree.

#[path = "../../../compositor/src/state/attach.rs"]
pub mod attach;
#[path = "../../../compositor/src/state/cursor.rs"]
pub mod cursor;
#[path = "../../../compositor/src/state/cursor_pick.rs"]
pub mod cursor_pick;
#[path = "../../../compositor/src/state/damage/mod.rs"]
pub mod damage;
#[path = "../../../compositor/src/state/damage_bound.rs"]
pub mod damage_bound;
#[path = "../../../compositor/src/state/display_rule.rs"]
pub mod display_rule;
#[path = "../../../compositor/src/state/indicator.rs"]
pub mod indicator;
#[path = "../../../compositor/src/state/lid_rule.rs"]
pub mod lid_rule;
#[path = "../../../compositor/src/state/raise_rule.rs"]
pub mod raise_rule;
#[path = "../../../compositor/src/state/scene/mod.rs"]
pub mod scene;
#[path = "../../../compositor/src/state/scene_raise.rs"]
pub mod scene_raise;
#[path = "../../../compositor/src/state/scene_remove.rs"]
pub mod scene_remove;
#[path = "../../../compositor/src/state/scene_submit.rs"]
pub mod scene_submit;
#[path = "../../../compositor/src/state/trusted_rule.rs"]
pub mod trusted_rule;
#[path = "../../../compositor/src/state/visible.rs"]
pub mod visible;
#[path = "../../../compositor/src/state/visible_drop.rs"]
pub mod visible_drop;
