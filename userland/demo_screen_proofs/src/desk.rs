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

//! The desktop shell's two surfaces for a `w` x `h` canvas, painted by its
//! own code: the desk (its icons, under every window) and the chrome (the
//! menubar and the dock, over them). The context is built by the shell's own
//! constructor over host memory; no compositor, window manager or router is
//! running, so nothing it would learn from them (open windows, tray items)
//! is drawn.

use alloc::vec::Vec;

use crate::render::chrome::paint_chrome;
use crate::setup::prime::overlay::Overlay;
use crate::setup::prime::peers::Peers;
use crate::setup::prime::run::build_context::build_context;

pub struct Desk {
    pub desk: Vec<u32>,
    pub chrome: Vec<u32>,
}

fn surface(w: u32, h: u32, pixels: &mut Vec<u32>) -> Overlay {
    let byte_len = u64::from(w) * u64::from(h) * 4;
    Overlay { width: w, height: h, stride: w * 4, byte_len, backing_va: pixels.as_mut_ptr() as u64 }
}

pub fn paint(w: u32, h: u32) -> Desk {
    let words = (w * h) as usize;
    let (mut desk, mut chrome) = (alloc::vec![0u32; words], alloc::vec![0u32; words]);
    let peers = Peers {
        compositor_port: 0,
        input_router_port: 0,
        wm_port: 0,
        wallpaper_port: 0,
        market_port: 0,
    };
    let (o, d) = (surface(w, h, &mut chrome), surface(w, h, &mut desk));
    let mut ctx = build_context(&peers, &o, &d, false);
    paint_chrome(&mut ctx);
    Desk { desk, chrome }
}
