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


//! Short `.anyone` names: the fork's naming layer (src/feature/anyone and
//! dirparse/anyone_hosts_parse.c), read strictly.
//!
//! A short name such as `dns-live-1.anyone.anyone` is not an address. It is
//! a line in a list that six Anyone DNS onion services publish at
//! `/tld/anyone`, signed with one of their identity keys. A name is
//! therefore only as trustworthy as that list. An address carries the
//! service's own key, so a stream to it can only reach that service;
//! whoever controls the DNS services' keys decides where a name points.
//! net.anon says so wherever a name is resolved (NAME_NOTICE), and keeps
//! three rules the fork's client does not:
//!
//! - A list is used only when its signature verifies under one of the six
//!   hardcoded keys and it has not expired. The fork's client uses an
//!   unsigned or badly signed list "with caution"; this one never does.
//! - A list never replaces a newer one, so a replayed old list cannot roll
//!   names back.
//! - Within a boot, a name keeps the first service it resolved to. If a
//!   later list points it elsewhere, the name is refused rather than
//!   followed. The pins are in memory only: this capsule has no storage,
//!   and a name is never written anywhere.

pub mod defaults;
pub mod document;
pub mod pins;

pub use document::{verify, NameList};
pub use pins::{may_replace, Pins, Resolve};

/// What a caller shows beside anything reached by a short name.
pub const NAME_NOTICE: &str = "Reached by a short .anyone name from the list the Anyone DNS services sign. \
A short name is weaker than the full address: the list decides where it points. \
NONOS refuses a name that changes service within a boot.";

/// The list's own size bound. The fork's default DNSMappingFileMaxSize is
/// larger; a list this capsule cannot hold is refused, not truncated.
pub const LIST_MAX: usize = 64 * 1024;
/// Names held from one list.
pub const NAMES_MAX: usize = 4096;
/// Where the services publish it (ANYONE_HOSTS_FETCH_PATH).
pub const FETCH_PATH: &str = "/tld/anyone";

/// `host` lowercased with its trailing root dots stripped: the form names
/// are compared in, as the fork lowercases a SOCKS address before mapping
/// it.
pub fn normal(host: &[u8]) -> alloc::vec::Vec<u8> {
    let mut h = host;
    while let Some(shorter) = h.strip_suffix(b".") {
        h = shorter;
    }
    h.to_ascii_lowercase()
}

extern crate alloc;
