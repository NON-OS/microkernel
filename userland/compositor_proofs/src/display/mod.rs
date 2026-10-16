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

//! What the compositor decides about displays: when to ask a name on GOP,
//! what a poll does, who may send a hint, layouts and scales, the cursor
//! plane, colour and accessibility, and the trusted band's indicator.

#[path = "../../../compositor/src/display/access_rule.rs"]
pub mod access_rule;
#[path = "../../../compositor/src/display/access_wire.rs"]
pub mod access_wire;
#[path = "../../../compositor/src/display/arrange_wire.rs"]
pub mod arrange_wire;
#[path = "../../../compositor/src/display/backoff.rs"]
pub mod backoff;
#[path = "../../../compositor/src/display/colour_rule.rs"]
pub mod colour_rule;
#[path = "../../../compositor/src/display/cursor_rule.rs"]
pub mod cursor_rule;
#[path = "../../../compositor/src/display/cvd_filter.rs"]
pub mod cvd_filter;
#[path = "../../../compositor/src/display/cvd_rule.rs"]
pub mod cvd_rule;
#[path = "../../../compositor/src/display/first_frame_rule.rs"]
pub mod first_frame_rule;
#[path = "../../../compositor/src/display/head.rs"]
pub mod head;
#[path = "../../../compositor/src/display/head_rule.rs"]
pub mod head_rule;
#[path = "../../../compositor/src/display/hint_rule.rs"]
pub mod hint_rule;
#[path = "../../../compositor/src/display/indicator_rule.rs"]
pub mod indicator_rule;
#[path = "../../../compositor/src/display/layout_rule.rs"]
pub mod layout_rule;
#[path = "../../../compositor/src/display/link.rs"]
pub mod link;
#[path = "../../../compositor/src/display/m3.rs"]
pub mod m3;
#[path = "../../../compositor/src/display/magnifier_rule.rs"]
pub mod magnifier_rule;
#[path = "../../../compositor/src/display/mirror_rule.rs"]
pub mod mirror_rule;
#[path = "../../../compositor/src/display/night_rule.rs"]
pub mod night_rule;
#[path = "../../../compositor/src/display/notice_rule.rs"]
pub mod notice_rule;
#[path = "../../../compositor/src/display/poll_rule.rs"]
pub mod poll_rule;
#[path = "../../../compositor/src/display/resize_rule.rs"]
pub mod resize_rule;
#[path = "../../../compositor/src/display/scale_rule.rs"]
pub mod scale_rule;
#[path = "../../../compositor/src/display/screenshot_rule.rs"]
pub mod screenshot_rule;
#[path = "../../../compositor/src/display/srgb_rule.rs"]
pub mod srgb_rule;
