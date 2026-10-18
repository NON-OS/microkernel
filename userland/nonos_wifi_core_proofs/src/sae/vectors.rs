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

//! The IEEE Std 802.11-2020 Annex J.10 hunting-and-pecking vector (sources in
//! `sae::vector_tests`), and the commit it builds from the given rand and mask.

use crate::hex::h;
use nonos_wifi_core::sae::commit::Commit;
use nonos_wifi_core::sae::hnp::derive_pwe;

pub fn a32(v: &[u8]) -> [u8; 32] {
    v.try_into().unwrap()
}

pub const ADDR1: [u8; 6] = [0x4d, 0x3f, 0x2f, 0xff, 0xe3, 0x87];
pub const ADDR2: [u8; 6] = [0xa5, 0xd8, 0xaa, 0x95, 0x8e, 0x3c];
pub const PW: &[u8] = b"mekmitasdigoat";

pub const LOCAL_RAND: &str = "992465fd3daa3c60aa6565b7f62a2a7f2e12dd12f198faf4fbed89d7ff1ace94";
pub const LOCAL_MASK: &str = "9507a90f777a044d6a0830b91ea3d5dd70bece44e1acffb86983b5e1bf9fb322";
pub const LOCAL_COMMIT: &str = "
    1300
    2e2c0f0db52440ad146d967114ce005ce1eab0aa2c2e5c2871b774f6c2575c65
    d5ad9e00829707aa36ba8b859738fc961d08243505f47c035376d7ac4bc8d7b9
    5083bf43827d0fc31ed778dd3671fd21a46d1091d64b6f9a1e1272621325dbe1";
pub const PEER_COMMIT: &str = "
    1300
    591b96f3397fb945100848e7b550543b6720d88337ee93fc49fd6df7e08b5223
    e71b9bb048d3873f20556953a96c91536fd8ee6ca9b4a68a148b056a909be03e
    83ae208f60f8ef5537858074db06687032399862999b511e0a1552a5fea317c2";
pub const KCK: &str = "1e733f6d9bd53256287304338831b09a39406d121017073a5c30db36f36cb81a";
pub const PMK: &str = "4e4dfab1a2dd8ac1a91790f953faaa452ae5c6873ab75b63605ba663f8a7fe59";
pub const PMKID: &str = "8747a600eea3f9f22475df58ca1e5498";

pub fn hnp_commit() -> Commit {
    // The stand-in password only has to be as long as the real one.
    let stub = [0x5Au8; 14];
    let pwe = derive_pwe(PW, &stub, &ADDR1, &ADDR2).expect("a PWE is found");
    Commit::new(pwe, &a32(&h(LOCAL_RAND)), &a32(&h(LOCAL_MASK))).expect("rand and mask are valid")
}
