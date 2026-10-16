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

//! The engine as a display-wire device (`nonos_gfx_serve::Display`): every
//! taken-over output, each with two scanout surfaces filled from the shadow
//! by FLUSH and flipped at the vertical blank.

mod brightness;
mod brightness_ops;
mod brightness_source;
mod brightness_wire;
mod copy;
mod cursor;
mod ddc_bus;
mod ddc_of;
mod describe;
mod frames;
mod head;
mod health;
mod intel;
mod link_keep;
mod mode_light;
mod mode_set;
mod modes;
mod modeset;
mod monitor;
mod monitor_read;
mod panel_fits;
mod panel_of;
mod plug;
mod plug_edge;
mod plug_poll;
mod plug_watch;
mod red_first;
mod rights;
mod timing_pick;
mod vcp_wire;
mod wire;

pub use brightness::Brightness;
pub use brightness_wire::{answer as brightness_answer, Rights, MAX_REPLY as BRIGHTNESS_REPLY};
pub use copy::copy;
pub use frames::Frames;
pub use head::Head;
pub use intel::{Intel, FLIP_US, MAX_HEADS};
pub use modeset::Modeset;
pub use monitor::Monitor;
pub use monitor_read::read as read_edid;
pub use panel_fits::fits;
pub use panel_of::{panel, panel_of};
pub use plug::Plug;
pub use plug_poll::POLL_US;
pub use red_first::{red_first, swap_red_blue};
pub use timing_pick::{matching, pick};
pub use vcp_wire::OP_VCP;
