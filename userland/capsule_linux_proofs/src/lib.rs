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

mod host_doubles;
pub use host_doubles::{clamp, private};

#[path = "../../../src/userspace/capsule_linux/family.rs"]
pub mod listing_family;

#[path = "../../capsule_linux/src/linux/file/family.rs"]
pub mod family;

#[path = "../../capsule_linux/src/linux/file/root.rs"]
pub mod root;

#[path = "../../capsule_linux/src/linux/file/resolve.rs"]
pub mod resolve;

#[path = "../../capsule_linux/src/linux/file/models/name.rs"]
pub mod model_name;

#[path = "../../capsule_linux/src/linux/file/dev_metrics_parse.rs"]
pub mod metrics_parse;

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

pub mod calls;
pub mod console;
pub mod pinned;
pub use calls::{exec_shebang, sigframe, sigframe_build, sigframe_read, sigtimer};

#[cfg(test)]
pub mod image;

/// The installer's parsers, which read bytes fetched off a network.
pub mod install;

#[cfg(test)]
mod tests;
