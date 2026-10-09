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

//! The shipped source, included and exercised.

extern crate alloc;

#[path = "../../capsule_linux/src/linux/wayland/wire.rs"]
pub mod wire;
#[path = "../../capsule_linux/src/linux/wayland/args.rs"]
pub mod args;
#[path = "../../capsule_linux/src/linux/wayland/object.rs"]
pub mod wayland_object;
#[path = "../../capsule_linux/src/linux/wayland/frame_len.rs"]
pub mod frame_len;
#[path = "../../capsule_linux/src/linux/wayland/window_damage.rs"]
pub mod window_damage;
/* A guest window full screen and back, and taken down when it ends. */
#[path = "../../capsule_linux/src/linux/wayland/fit.rs"]
pub mod fit;
#[path = "../../capsule_linux/src/linux/wayland/ops.rs"]
pub mod ops;
#[path = "../../capsule_linux/src/linux/wayland/out.rs"]
pub mod out;
#[path = "../../capsule_linux/src/linux/wayland/configure.rs"]
pub mod configure;
#[path = "../../capsule_linux/src/linux/wayland/reshape.rs"]
pub mod reshape;
#[path = "../../capsule_linux/src/linux/wayland/scene_pixels.rs"]
pub mod scene_pixels;
#[path = "../../capsule_linux/src/linux/wayland/window_life.rs"]
pub mod window_life;

mod host_doubles;
pub use host_doubles::{clamp, private};

#[path = "../../../src/userspace/capsule_linux/family.rs"]
pub mod listing_family;

#[path = "../../capsule_linux/src/linux/file/family.rs"]
pub mod family;
/* A program's pin, hashed a window at a time (`hash_windows_tests`). */
#[path = "../../capsule_linux/src/linux/file/hash_windows.rs"]
pub mod hash_windows;
#[path = "../../capsule_linux/src/linux/file/root.rs"]
pub mod root;
#[path = "../../capsule_linux/src/linux/file/resolve.rs"]
pub mod resolve;
#[path = "../../capsule_linux/src/linux/file/models/name.rs"]
pub mod model_name;
#[path = "../../capsule_linux/src/linux/file/models/stat_size.rs"]
pub mod stat_size;
#[path = "../../capsule_linux/src/linux/file/dev_metrics_parse.rs"]
pub mod metrics_parse;
/* What the `linux` command says when the store has nothing (`store_why_tests`). */
#[path = "../../capsule_linux/src/linux/store_why.rs"]
pub mod store_why;
#[path = "../../capsule_linux/src/linux/file/dir_children.rs"]
pub mod dir_children;
#[path = "../../capsule_linux/src/linux/file/dirent.rs"]
pub mod dirent;
#[path = "../../capsule_linux/src/linux/file/meta/statbuf/mod.rs"]
pub mod statbuf;

#[path = "../../capsule_linux/src/linux/net/host_body.rs"]
pub mod host_body;

/*
 * net.sockets' own reader for a connect-by-host body, mounted at the crate
 * paths it names, so the capsule's encoder is held to the real parser.
 */
#[path = "../../capsule_net_sockets/src/protocol/errno.rs"]
pub mod protocol;
pub mod server;

#[path = "../../capsule_linux/src/linux/net/route.rs"]
pub mod route;

/* The statuses the capsule reads back from net.sockets, held to the server's. */
#[path = "../../capsule_linux/src/linux/net/ops.rs"]
pub mod net_ops;

/*
 * The network a guest's stream leaves through is nonos_route_link's pick, as
 * it ships. At the crate root, where its own `crate::pick` paths look for it.
 */
#[path = "../../nonos_route_link/src/pick.rs"]
pub mod pick;

/*
 * net.anon's own operations, statuses and END reasons, which the capsule's
 * client of its handle front is held to.
 */
#[path = "../../capsule_net_anon/src/protocol/ops.rs"]
pub mod anon_server_ops;
#[path = "../../capsule_net_anon/src/protocol/errno.rs"]
pub mod anon_server_errno;
#[path = "../../capsule_net_anon/src/stream/end.rs"]
pub mod anon_server_end;

#[path = "../../capsule_linux/src/linux/applets.rs"]
pub mod applets;

pub mod calls;
pub mod console;
/* The call rules, at the crate paths they name in the capsule. */
pub mod linux;
pub mod models;
pub use calls::{exec_shebang, sigframe, sigframe_build, sigframe_read, sigtimer};

#[cfg(any(test, fuzzing))]
pub mod image;

/// The installer's parsers, which read bytes fetched off a network.
pub mod install;

#[cfg(test)]
mod tests;
