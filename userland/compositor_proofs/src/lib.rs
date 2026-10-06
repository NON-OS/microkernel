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

//! Host proofs for the compositor's trusted-path primitives. The `#[path]`
//! includes pull in the real production source so the tests pin the shipping
//! behavior, not a copy. `state::damage` is re-exported here so the blitter's
//! `crate::state::damage::Rect` path resolves exactly as it does in-tree.
//! The scene table and the submit, raise and remove steps sit beside the
//! damage accumulator at the crate root, so their `super::` paths resolve
//! to the same neighbours they have in the compositor's `state`.

#[path = "../../compositor/src/state/attach.rs"]
pub mod attach;

#[path = "../../compositor/src/state/damage.rs"]
pub mod damage;

#[path = "../../compositor/src/state/scene/mod.rs"]
pub mod scene;

#[path = "../../compositor/src/state/raise_rule.rs"]
pub mod raise_rule;

#[path = "../../compositor/src/state/scene_raise.rs"]
pub mod scene_raise;

#[path = "../../compositor/src/state/scene_remove.rs"]
pub mod scene_remove;

#[path = "../../compositor/src/state/scene_submit.rs"]
pub mod scene_submit;

#[path = "../../compositor/src/state/visible.rs"]
pub mod visible;

pub mod state {
    pub mod damage {
        pub use crate::damage::{DamageAccumulator, Rect};
    }
    pub use crate::{attach, scene, visible};
}

// A damaged rectangle composed onto real pixels, as a frame composes it,
// with the arrow cursor drawn last.
#[path = "."]
pub mod frame_pacer {
    #[path = "../../compositor/src/frame_pacer/compose.rs"]
    pub mod compose;
    #[path = "../../compositor/src/frame_pacer/cursor.rs"]
    pub mod cursor;
}

#[path = "../../compositor/src/frame_pacer/drain_damage.rs"]
pub mod drain_damage;

#[path = "../../compositor/src/sw_blitter/mod.rs"]
pub mod sw_blitter;

// The wire every request arrives on: the header decode the drain runs before
// dispatch, and the encoders a refusal is answered with.
#[path = "../../compositor/src/protocol/mod.rs"]
pub mod protocol;

#[cfg(test)]
mod attach_tests;
#[cfg(test)]
mod band_tests;
#[cfg(test)]
mod blitter_tests;
#[cfg(test)]
mod damage_tests;
#[cfg(test)]
mod downscale_tests;
#[cfg(test)]
mod floor_tests;
#[cfg(test)]
mod foreign_pixel_tests;
#[cfg(test)]
mod frame_model;
#[cfg(test)]
mod present_retry_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod raise_tests;
#[cfg(test)]
mod repaint_tests;
#[cfg(test)]
mod stacking_tests;
#[cfg(test)]
mod upscale_tests;
