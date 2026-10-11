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

//! The installer's pure parsers, included from the capsule.

pub mod auth;

#[path = "../../../capsule_linux/src/linux/install/http_reply.rs"]
pub mod http_reply;

/* How a mirror is named and asked, and how its reply is read off a stream
 * through the chosen network: the decisions, without the stream. */
#[path = "../../../capsule_linux/src/linux/install/mirror.rs"]
pub mod mirror;

#[path = "../../../capsule_linux/src/linux/install/market_arg.rs"]
pub mod market_arg;
#[path = "../../../capsule_linux/src/linux/install/market_check.rs"]
pub mod market_check;
#[path = "../../../capsule_linux/src/linux/install/proof_names.rs"]
pub mod proof_names;
#[path = "../../../capsule_linux/src/linux/install/route_read.rs"]
pub mod route_read;

#[path = "../../../capsule_linux/src/linux/install/tools.rs"]
pub mod tools;

#[path = "../../../capsule_linux/src/linux/install/pkg.rs"]
pub mod pkg;

#[path = "../../../capsule_linux/src/linux/install/index.rs"]
pub mod index;

#[path = "../../../capsule_linux/src/linux/install/hex.rs"]
pub mod hex;

pub mod deb;
pub mod pacman;
pub mod pgp;

#[path = "../../../capsule_linux/src/linux/install/manifest/mod.rs"]
pub mod manifest;
#[path = "../../../capsule_linux/src/linux/install/member/mod.rs"]
pub mod member;
/* The link table's writer, which only the tests here call. */
#[cfg(test)]
#[path = "../../../capsule_linux/src/linux/install/place_links/mod.rs"]
pub mod place_links;
#[cfg(test)]
mod place_links_more_tests;
#[cfg(test)]
mod place_links_tests;

mod escrow_files;
#[cfg(test)]
pub mod image_list;
mod tar_files;
#[path = "../../../capsule_linux/src/linux/install/unpacked.rs"]
pub mod unpacked;
pub use escrow_files::*;
pub use tar_files::*;
