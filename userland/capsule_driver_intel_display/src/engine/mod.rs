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

pub mod access;
pub mod aux_ch;
pub mod aux_regs;
pub mod backlight;
pub mod cdclk;
pub mod cursor;
pub mod dc;
pub mod ddb;
pub mod ddi;
pub mod dpll;
pub mod dpll_cfgcr;
pub mod dpll_ctrl;
pub mod dpll_hdmi;
pub mod dpll_split;
pub mod error;
pub mod flip;
pub mod gen;
pub mod ggtt;
pub mod ggtt_pte;
pub mod ggtt_range;
pub mod gmbus;
pub mod gmbus_regs;
pub mod gmbus_take;
pub mod hdmi;
pub mod hotplug;
pub mod mn;
pub mod modeset;
pub mod modeset_off;
pub mod pcode;
pub mod pipe;
pub mod plane;
pub mod plane_kind;
pub mod plane_read;
pub mod plane_regs;
pub mod plane_stride;
pub mod port;
pub mod port_link;
pub mod port_start;
pub mod port_stop;
pub mod power;
pub mod pps;
pub mod regs;
pub mod scanout;
pub mod scanout_start;
pub mod scanout_stop;
pub mod takeover;
pub mod timing;
pub mod timing_write;
pub mod trans_func;
pub mod vblank;
pub mod wait;
pub mod wm;
pub mod wm_write;
