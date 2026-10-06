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

//! Host proofs for the wallpaper catalog service's request path. The desktop
//! and settings read wallpapers through it, and any process holding the
//! endpoint can send it any bytes. The `#[path]` includes pull in the real
//! frame decode, handlers, replies and the catalog itself under the `crate::`
//! paths their files name each other by.

extern crate alloc;

// The header, ops, limits and errnos.
#[path = "../../capsule_wallpaper_catalog/src/protocol/mod.rs"]
pub mod protocol;

// The wallpapers the service hands out: their pins, and the one it holds,
// read out of the store's collection.
#[path = "../../capsule_wallpaper_catalog/src/catalog/mod.rs"]
pub mod catalog;

// The vfs stream the catalog reads the collection through, answered here from
// the real wallpaper files, packed as tools/nonos-wallpaper-pack packs them.
// It answers to the app skeleton's name, as the catalog calls it by that.
extern crate self as nonos_app_skeleton;
pub mod clients;
pub mod collection;

// The frame decode, the handlers and the replies; not the receive loop.
pub mod server;

// The wallpaper service's side of it: its catalog client, which fetches a
// wallpaper a chunk per call and keeps what came over between tries, and the
// decoder it paints from. Its calls reach `serve` through the stand-in
// kernel's `mk_ipc_call_timeout`.
#[path = "../../capsule_wallpaper/src/catalog_client/mod.rs"]
pub mod wallpaper_client;
#[path = "../../capsule_wallpaper/src/paint/decode_jpeg.rs"]
pub mod wallpaper_decode;
// What it says on the serial line about the wallpaper chosen.
#[path = "../../capsule_wallpaper/src/subscriber/say.rs"]
pub mod wallpaper_say;

// The job its worker thread runs (fetch, then decode) and the plan its
// service thread keeps about it: which wallpaper to start, what a stopped
// job kept, and whether a finished one is still wanted. The job names the
// client and the decoder by the service's own paths.
pub use wallpaper_client as catalog_client;
pub mod paint {
    pub use crate::wallpaper_decode::{decode_jpeg, DecodedImage};
}
// When it asks the compositor again: a scene submit or a damage commit that
// did not go through is retried at a growing interval, never given up on.
#[path = "../../capsule_wallpaper/src/state/backoff.rs"]
pub mod wallpaper_backoff;
#[path = "../../capsule_wallpaper/src/subscriber/job"]
pub mod wallpaper_job {
    pub mod fetch;
    pub mod plan;
}

#[cfg(test)]
mod serve_tests;
#[cfg(test)]
mod pin_tests;
#[cfg(test)]
mod said_tests;
#[cfg(test)]
mod download_tests;
#[cfg(test)]
mod wallpaper_said_tests;
#[cfg(test)]
mod wallpaper_job_tests;
#[cfg(test)]
mod wallpaper_backoff_tests;
