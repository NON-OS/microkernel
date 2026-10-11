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

/*
 * Text for an intent's review: contract names, and hex as a person reads
 * it (an address whole, a 32-byte id as its head and tail).
 */

use alloc::string::String;

use nonos_nox_config::Contract;

const HEX: &[u8; 16] = b"0123456789abcdef";

pub fn hex(b: &[u8]) -> String {
    let mut s = String::from("0x");
    b.iter().for_each(|x| {
        s.push(HEX[(x >> 4) as usize] as char);
        s.push(HEX[(x & 15) as usize] as char);
    });
    s
}

/// 0x and the first 8 and last 6 hex digits of a 32-byte id.
pub fn id(b: &[u8; 32]) -> String {
    let full = hex(b);
    let mut s = String::from(&full[..10]);
    s.push_str("...");
    s.push_str(&full[full.len() - 6..]);
    s
}

pub fn contract_name(c: Contract) -> &'static str {
    match c {
        Contract::NoxToken => "the NOX token",
        Contract::Settlement => "NONOS Settlement",
        Contract::Escrow => "NONOS Escrow",
        Contract::PublisherBond => "NONOS Bond",
        Contract::ListingRegistry => "NONOS Listing registry",
        Contract::ProofAnchor => "NONOS Proof anchor",
        Contract::ShieldPool => "the Shield pool",
    }
}
