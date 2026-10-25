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

//! Host proofs for the wallpaper catalog's request path, which any holder of
//! the endpoint can send any bytes, and the wallpaper service's side of it.
//! The `#[path]` includes pull in the real code under its own `crate::` paths.

extern crate alloc;

// The header, ops, limits and errnos.
#[path = "../../capsule_wallpaper_catalog/src/protocol/mod.rs"]
pub mod protocol;
// The wallpapers it hands out: their pins, read out of the store's collection.
#[path = "../../capsule_wallpaper_catalog/src/catalog/mod.rs"]
pub mod catalog;

// The vfs stream the catalog reads, answered from the real wallpaper files
// packed as tools/nonos-wallpaper-pack packs them, under the skeleton's name.
extern crate self as nonos_app_skeleton;
pub mod clients;
pub mod collection;

// The frame decode, the handlers and the replies; not the receive loop.
pub mod server;

// The wallpaper service's side: its catalog client (a chunk per call, what
// came over kept between tries) and its decoder; the calls reach `serve`
// through the stand-in kernel's `mk_ipc_call_timeout`.
#[path = "../../capsule_wallpaper/src/catalog_client/mod.rs"]
pub mod wallpaper_client;
#[path = "../../capsule_wallpaper/src/paint/decode_jpeg.rs"]
pub mod wallpaper_decode;
#[path = "../../capsule_wallpaper/src/subscriber/say.rs"]
pub mod wallpaper_say;

// Its worker's job (fetch, then decode) and the plan its service thread
// keeps (which to start, what a stopped job kept, whether a finished one is
// still wanted), and the built-in picture a worker paints while the service
// answers (ink, then the picture). They name each other by the service's paths.
pub use wallpaper_client as catalog_client;
pub mod paint {
    pub use crate::wallpaper_blit as blit_argb;
    pub use crate::wallpaper_decode::{decode_jpeg, DecodedImage};
    pub use crate::wallpaper_fill::fill_argb;
}
#[path = "../../capsule_wallpaper/src/paint/blit_argb.rs"]
pub mod wallpaper_blit;
#[path = "../../capsule_wallpaper/src/paint/fill.rs"]
pub mod wallpaper_fill;
#[path = "../../capsule_wallpaper/src/first_paint/job.rs"]
pub mod wallpaper_first_paint;
// A scene submit or damage commit not answered is asked again, ever later.
#[path = "../../capsule_wallpaper/src/state/backoff.rs"]
pub mod wallpaper_backoff;
#[path = "../../capsule_wallpaper/src/subscriber/job"]
pub mod wallpaper_job {
    pub mod fetch;
    pub mod plan;
}

#[cfg(test)]
mod tests;
