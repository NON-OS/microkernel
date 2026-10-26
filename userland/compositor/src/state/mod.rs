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

pub mod attach;
pub mod attach_kernel;
pub mod context;
pub mod context_ask;
pub mod context_new;
pub mod cursor;
pub mod cursor_pick;
pub mod damage;
pub mod damage_bound;
pub mod display_rule;
pub mod focus;
pub mod indicator;
pub mod lid_rule;
pub mod looks;
pub mod primary;
pub mod raise_rule;
pub mod scene;
pub mod scene_raise;
pub mod scene_remove;
pub mod scene_submit;
pub mod trusted_rule;
pub mod visible;
pub mod visible_drop;

pub use attach::AttachCache;
pub use context::Context;
pub use cursor::CursorTracker;
pub use damage::DamageAccumulator;
pub use focus::FocusTable;
pub use primary::Primary;
pub use scene::SceneTable;
