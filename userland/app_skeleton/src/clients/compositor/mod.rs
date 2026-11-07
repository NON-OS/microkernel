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

mod content;
mod cursor_shape;
mod damage_commit;
mod display_info;
mod frame_time;
mod healthcheck;
mod present;
mod scanout_peer;
mod scene_remove;
mod scene_remove_layer;
mod scene_submit;
mod scene_submit_layer;

pub use content::{
    content_body, content_parts_body, layer_content, layer_content_parts, Content, CONTENT_OPAQUE,
    LAYER_CONTENT_LEN, LAYER_CONTENT_PARTS_LEN, OP_LAYER_CONTENT, OP_LAYER_CONTENT_PARTS,
};
pub use cursor_shape::cursor_shape;
pub use damage_commit::damage_commit;
pub use display_info::display_info;
pub use frame_time::{frame_time, FrameTime};
pub use healthcheck::healthcheck;
pub use present::{present, present_layer, present_layer_parts};
pub use scanout_peer::scanout_peer;
pub use scene_remove::scene_remove;
pub use scene_remove_layer::scene_remove_layer;
pub use scene_submit::scene_submit;
pub use scene_submit_layer::{scene_submit_layer, KeyedLayer};
