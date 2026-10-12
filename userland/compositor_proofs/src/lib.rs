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

#[path = "../../compositor/src/state/indicator.rs"]
pub mod indicator;

pub mod state {
    pub mod damage {
        pub use crate::damage::{DamageAccumulator, Rect};
    }
    pub use crate::{attach, indicator, scene, visible};
}

// A damaged rectangle composed onto real pixels, as a frame composes it,
// with the arrow cursor drawn last.
#[path = "."]
pub mod frame_pacer {
    #[path = "../../compositor/src/frame_pacer/canvas_copy.rs"]
    pub mod canvas_copy;
    #[path = "../../compositor/src/frame_pacer/compose.rs"]
    pub mod compose;
    #[path = "../../compositor/src/frame_pacer/cursor.rs"]
    pub mod cursor;
    #[path = "../../compositor/src/frame_pacer/cursor_shape.rs"]
    pub mod cursor_shape;
    #[path = "../../compositor/src/frame_pacer/indicator_band.rs"]
    pub mod indicator_band;
}

#[path = "../../compositor/src/frame_pacer/drain_damage.rs"]
pub mod drain_damage;

#[path = "../../compositor/src/frame_pacer/drain_batch.rs"]
pub mod drain_batch;

#[path = "../../compositor/src/sw_blitter/mod.rs"]
pub mod sw_blitter;

// The wire every request arrives on: the header decode the drain runs before
// dispatch, and the encoders a refusal is answered with.
#[path = "../../compositor/src/protocol/mod.rs"]
pub mod protocol;

// What the compositor decides about a display driver on the wire: when to
// ask a name while on GOP, what a poll does, and who may send a size hint.
#[path = "."]
pub mod display {
    #[path = "../../compositor/src/display/backoff.rs"]
    pub mod backoff;
    #[path = "../../compositor/src/display/cursor_rule.rs"]
    pub mod cursor_rule;
    #[path = "../../compositor/src/display/head.rs"]
    pub mod head;
    #[path = "../../compositor/src/display/head_rule.rs"]
    pub mod head_rule;
    #[path = "../../compositor/src/display/hint_rule.rs"]
    pub mod hint_rule;
    #[path = "../../compositor/src/display/indicator_rule.rs"]
    pub mod indicator_rule;
    #[path = "../../compositor/src/display/layout_rule.rs"]
    pub mod layout_rule;
    #[path = "../../compositor/src/display/link.rs"]
    pub mod link;
    #[path = "../../compositor/src/display/scale_rule.rs"]
    pub mod scale_rule;
    #[path = "../../compositor/src/display/screenshot_rule.rs"]
    pub mod screenshot_rule;
    #[path = "../../compositor/src/display/poll_rule.rs"]
    pub mod poll_rule;
    #[path = "../../compositor/src/display/resize_rule.rs"]
    pub mod resize_rule;
}

/// The kernel's display list and hint sources, which the compositor mounts.
#[allow(dead_code)]
#[path = "../../../src/services/registry/displays.rs"]
pub mod displays;

#[cfg(test)]
mod attach_tests;
#[cfg(test)]
mod backoff_tests;
#[cfg(test)]
mod band_tests;
#[cfg(test)]
mod batch_tests;
#[cfg(test)]
mod blitter_tests;
#[cfg(test)]
mod cursor_plane_tests;
#[cfg(test)]
mod cursor_rule_tests;
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
mod head_tests;
#[cfg(test)]
mod hint_tests;
#[cfg(test)]
mod indicator_band_tests;
#[cfg(test)]
mod indicator_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod mirror_tests;
#[cfg(test)]
mod privacy_tests;
#[cfg(test)]
mod scale_tests;
#[cfg(test)]
mod screenshot_tests;
#[cfg(test)]
mod poll_tests;
#[cfg(test)]
mod present_retry_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod raise_tests;
#[cfg(test)]
mod repaint_tests;
#[cfg(test)]
mod resize_fit_tests;
#[cfg(test)]
mod resize_tests;
#[cfg(test)]
mod stacking_tests;
#[cfg(test)]
mod upscale_tests;
