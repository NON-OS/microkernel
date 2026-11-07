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

/// Room for a few dozen windows across every client: the desktop's own
/// layers, a handful of applications, and the dialogs and menus each of them
/// opens. The attach cache maps as many surfaces (attach.rs MAX_ATTACH).
pub const MAX_LAYERS: usize = 64;

/// The most layers one process may hold, one per z band. An application holds
/// one; the desktop shell holds two (its desktop under the windows and its
/// chrome over them). The cap keeps one client from filling the table.
/// One client's share: a main window, its dialogs and the popups open over
/// them, each a layer of its own. A Linux guest behind the Wayland bridge
/// holds one layer per toplevel and per popup.
pub const MAX_LAYERS_PER_OWNER: usize = 16;

/// A second surface drawn over a layer's own, from a corner inside it, and
/// cut to it: a client's buffer shown where the client drew it, under the
/// frame the layer's own surface holds (OP_LAYER_CONTENT). Handle 0 is none.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Content {
    pub handle: u64,
    /// Its corner on the layer.
    pub x: u32,
    pub y: u32,
    /// Every pixel is drawn as opaque, its alpha byte not read: an XRGB
    /// buffer, whose fourth byte is anything at all.
    pub opaque: bool,
    /// The box of the surface drawn from the corner, (x, y, width, height)
    /// in its pixels: all of a buffer, or a window's part of one that also
    /// holds the shadow its client draws round it.
    pub view: (u32, u32, u32, u32),
}

#[derive(Clone, Copy, Default)]
pub struct Layer {
    pub owner_pid: u32,
    pub surface_handle: u64,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub z: u32,
    /// Raise order inside the layer's z band: the layer raised (or first
    /// submitted) most recently has the highest value and draws over every
    /// other layer of its band. The window manager raises a window on every
    /// click, so this mirrors its stack and the window drawn on top is the
    /// window its hit test gives the next click to.
    pub stack: u32,
    pub in_use: bool,
    /// The window manager's id for the window this layer draws, as its owner
    /// gave it with a keyed submit; 0 for a layer submitted the old way.
    pub window_id: u32,
    /// Submitted by handle (submit_keyed): the project may hold several such
    /// layers in one band, each named by its surface handle. A layer
    /// submitted the old way is its owner's one layer in its band.
    pub keyed: bool,
    pub miss_count: u16,
    /// The pointer's shape over this layer, as its owner asked
    /// (frame_pacer/cursor_shapes.rs); None until it asks. A layer that never
    /// asks, as the shell's chrome over every window, leaves the pointer to
    /// the layers under it.
    pub cursor: Option<u8>,
    /// What is drawn over the layer's own surface, kept across a move, a
    /// resubmit and a present: only OP_LAYER_CONTENT or the layer's end
    /// changes it.
    pub content: Content,
    /// The last present of the surface the layer shows said every pixel of
    /// it is opaque, so nothing under the layer shows through it. Only a
    /// present sets it; a submit says nothing of the pixels and clears it.
    pub opaque: bool,
}
