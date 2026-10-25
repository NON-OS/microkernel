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

//! The compositor's scene and damage, the input router's press grab, and
//! app_skeleton's and the toolkit's types at the paths the runner names.

// The compositor's scene, damage and the submit and raise steps, side by side
// as they sit in its `state` module.
#[path = "../../../compositor/src/state/damage/mod.rs"]
pub mod damage;

#[path = "../../../compositor/src/state/scene/mod.rs"]
pub mod scene;

#[path = "../../../compositor/src/state/scene_raise.rs"]
pub mod scene_raise;

#[path = "../../../compositor/src/state/scene_submit.rs"]
pub mod scene_submit;

// The input router's press grab.
#[path = "../../../capsule_input_router/src/state/press.rs"]
pub mod press;

// app_skeleton's frame: the drag and the press routing inside a window, under
// `runner` as in app_skeleton, with the event types at `crate::input`.
pub mod input {
    pub use nonos_app_skeleton::{InputEvent, InputKind};
}

// The delivery header magic, at the `crate::wire` path app_skeleton's
// delivery decode names it by.
pub mod wire {
    pub use nonos_app_skeleton::wire::NINP_MAGIC;
}

// The toolkit's painting, at the `crate::paint` path app_skeleton's runner
// files name it by.
pub mod paint {
    pub use nonos_toolkit::paint::*;
}
