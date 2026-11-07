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

//! The client's scene, and the NONOS surface it ends up on.

use alloc::vec::Vec;

use super::clip::Clip;
use super::held_keys::HeldKeys;
use super::popup::stack::Popups;
use super::scene_window::Window;
use super::state::{Buffer, Pool, Surface};
use super::subsurface::state::Subs;

pub struct Scene {
    pub pools: Vec<Pool>,
    pub buffers: Vec<Buffer>,
    pub surfaces: Vec<Surface>,
    /// Serial for configure events, which a client echoes back.
    pub serial: u32,
    /// The window shown now, on the guest's one layer and window manager
    /// window (scene_window.rs).
    pub win: Window,
    /// The guest's other mapped toplevels' windows, the latest last, each
    /// kept whole to be shown again when the one shown goes (window_front/).
    pub behind: Vec<Window>,
    /// Which toplevel surface each decoration object was made for, so its
    /// mode is that toplevel's (decoration.rs, kde_decoration.rs).
    pub decorations: Vec<(u32, u32)>,
    pub pointer: Option<u32>,
    pub keyboard: Option<u32>,
    /// The surface the pointer and the keyboard were last told they are on
    /// (focus_step.rs).
    pub pointer_on: Option<u32>,
    /// The serial of the pointer's latest enter, which a cursor request
    /// names, and the compositor's shape now (input/cursor.rs).
    pub pointer_entered: u32,
    /// Where on that surface the pointer last was, which a pointer made
    /// while it is there is told.
    pub pointer_at: (i32, i32),
    pub cursor: u8,
    pub keyboard_on: Option<u32>,
    /// Presentation feedbacks waiting for the compositor to show the frame
    /// they were asked for (presentation_wait.rs).
    pub presenting: Vec<super::presentation_wait::Waiting>,
    /// The display's refresh period as the compositor last reported it, in
    /// nanoseconds; 0 until it does (output.rs, commit/pace.rs).
    pub refresh_ns: u64,
    /// Whether the input router has taken this personality's subscription.
    pub subscribed: bool,
    /// The latest motion a backed-up client was not sent (input/latest.rs).
    pub motion_kept: super::input::latest::Latest<super::input::pump::Routed>,
    /// The surfaces told they are on the output (output_enter.rs).
    pub on_output: Vec<u32>,
    /// The keys down now, so a driver's repeat is not sent as a press.
    pub held: HeldKeys,
    /// Surfaces placed on the window's (subsurface/).
    pub subs: Subs,
    /// Menus and the like (popup/), and those in windows of their own.
    pub popups: Popups,
    pub popup_windows: Vec<super::popup::window::PopupWindow>,
    /// The guest's selections (clip/).
    pub clip: Clip,
    /// Its drags (dnd/).
    pub dnd: super::dnd::Dnd,
    /// The guest's buffers pinned to be shown in place (present/pinned/).
    pub pins: Vec<super::present::pinned::pins::Pin>,
    /// Buffers a layer drew, given back once the compositor lets them go.
    pub releasing: Vec<super::present::pinned::held::Held>,
    /// The guest's wl_regions, by id, as they were built (region.rs).
    pub regions: Vec<(u32, super::region_cover::Steps)>,
}
