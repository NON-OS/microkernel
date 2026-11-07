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

//! What the client has built: its pools, its buffers, its surfaces.

use alloc::vec::Vec;

pub struct Pool {
    pub id: u32,
    /// The size the client gave it, at create_pool or its last resize. Where
    /// its pixels are is asked of the memfd's spans at each read
    /// (present/source.rs), never kept: the client may unmap them.
    pub size: u64,
    /// The client's descriptor it was made over, which a resize looks at
    /// again: the client may have mapped it anew at another address.
    pub fd: u32,
    /// The memfd behind it, found again wherever it is mapped (memfd_map.rs).
    pub memfd: u32,
}

pub struct Buffer {
    pub id: u32,
    pub pool: u32,
    pub offset: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// XRGB8888, taken as opaque (opaque.rs); ARGB8888 otherwise.
    pub opaque: bool,
}

pub struct Surface {
    pub id: u32,
    /// The buffer attached but not yet committed.
    pub pending: Option<u32>,
    /// The xdg_surface wrapping it, once the client asks for one.
    pub xdg: Option<u32>,
    /// The xdg_toplevel made from that xdg_surface, once there is one.
    pub toplevel: Option<u32>,
    /// Frame callbacks owed, answered after the next present.
    pub frames: Vec<u32>,
    /// Those presented and held for their frame period (commit/pace.rs),
    /// and when the last were answered.
    pub held: Vec<u32>,
    pub last_done: Option<u64>,
    /// Set once a configure has been sent and acked.
    pub configured: bool,
    /// Its toplevel's first configure, owed at its next commit: the
    /// client's initial commit, which xdg-shell answers (toplevel/first.rs).
    pub initial: bool,
    /// A mode its toplevel asked for before that first configure, told in it.
    pub wants: Option<super::fit::Mode>,
    /// What its toplevel said of itself: a parent (set_parent), and its
    /// least and most size (set_min_size, set_max_size; 0 is no limit).
    pub parent: bool,
    pub limits: ((u32, u32), (u32, u32)),
    /// The part of the surface that is the window, set_window_geometry's
    /// x, y, width and height in its logical pixels: a client that draws
    /// its own shadow draws it outside. None until the client says.
    pub geometry: Option<(i32, i32, i32, i32)>,
    /// Its toplevel's title, and whether it draws its own frame, as it last
    /// said: its window's frame takes them whenever it is shown.
    pub title: Vec<u8>,
    pub own_frame: bool,
    /// wp_presentation feedbacks asked for its next commit (presentation.rs).
    pub feedback: Vec<u32>,
    /// Its buffer scale and viewport (present/scale_math.rs).
    pub scale: super::present::scale_math::Scale,
    /// What it said changed since its last commit, in buffer pixels, and
    /// whether a buffer was attached since (damage.rs).
    pub damage: super::commit::damage_show::damage::Damage,
    /// The boxes it said changed since then, kept apart (damage_boxes.rs).
    pub boxes: super::commit::damage_show::damage_boxes::Boxes,
    pub attached: bool,
    /// The opaque region set since its last commit, and the one it holds
    /// now, in surface pixels (region.rs).
    pub opaque_said: Option<super::region_cover::Steps>,
    pub opaque: super::region_cover::Steps,
}
