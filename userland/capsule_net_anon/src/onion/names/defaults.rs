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


//! The six DNS services the fork hardcodes (DEFAULT_ANON_DNS_MAPPING in
//! app/config/config.h). Their names map to them with the trust of this
//! code, and their keys are the only ones a list may be signed with.

extern crate alloc;

use alloc::vec::Vec;

use super::document::NameList;

/// The fork's default mapping, verbatim.
pub const MAPPING: [(&str, &str); 6] = [
    ("dns-stage-1.anyone.anyone", "hnsywhyh3zvvqzkmum7b3fxueii3bueeqjbwkfpngcqktxmubedrf5yd.anyone"),
    ("dns-stage-2.anyone.anyone", "xxfuq2xfwq7vxgadwywmtmfzeyk5j2oxhjhbn3onaq5h7yp7e3tpmkqd.anyone"),
    ("dns-stage-3.anyone.anyone", "xvtw2foswsovdutimyjo66zy3k26uehfcwdgrakut43cw4fto2djo2qd.anyone"),
    ("dns-live-1.anyone.anyone", "gadmrvl67444hgzrhsnhzknxaimfnzp6az3wq4d2j7hrf7th34elrrad.anyone"),
    ("dns-live-2.anyone.anyone", "kjlkfrfxquevo64qv4gssl3t52tiuay2muj7u4rox4llxboj4c4ypcid.anyone"),
    ("dns-live-3.anyone.anyone", "jntoblprbfgcpldwuzobmzsdjs6mtwtr3dtn3mtgdjnk6j7x2frcabad.anyone"),
];

/// The identity keys of the six services. An address that does not check
/// out is left out, which a proof holds is none of them.
pub fn signers() -> Vec<[u8; 32]> {
    MAPPING.iter().filter_map(|(_, address)| crate::onion::address::parse(address.as_bytes())).collect()
}

/// The six names as a list of their own, never expiring.
pub fn list() -> NameList {
    let entries = MAPPING
        .iter()
        .filter_map(|(name, address)| Some((name.as_bytes().to_vec(), crate::onion::address::parse(address.as_bytes())?)))
        .collect();
    NameList { published: 0, valid_until: u64::MAX, entries }
}
