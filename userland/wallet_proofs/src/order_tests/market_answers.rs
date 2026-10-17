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

//! The market's answers to a named purchase's four calls, as bytes the
//! way market.index writes them.

use nonos_market_proto::{Commerce, Readiness, ReleaseEscrow};

fn lp(o: &mut Vec<u8>, b: &[u8]) {
    o.extend((b.len() as u32).to_le_bytes());
    o.extend(b);
}

pub fn commerce_body(c: &Commerce) -> Vec<u8> {
    let mut o = vec![c.commerce_verdict, c.binding_verdict];
    [c.listing_id, c.release_id, c.release_hash].iter().for_each(|w| o.extend(w));
    o.extend(c.chain_id.to_le_bytes());
    lp(&mut o, &c.token_contract);
    lp(&mut o, &c.block);
    o
}

/// GET_RELEASE's reply: the release's id text, hashes, url, then empty
/// fields and lists (13 bytes: two counts of none, with 4 and 1 fixed
/// bytes), then the chunk root and length the market appends.
pub fn release_body(e: &ReleaseEscrow, version: &[u8]) -> Vec<u8> {
    let mut o = Vec::new();
    lp(&mut o, version);
    o.extend([0; 64]);
    lp(&mut o, &e.package_url);
    lp(&mut o, b"");
    o.extend([0; 13]);
    [b"", b""].iter().for_each(|f| lp(&mut o, *f));
    o.extend([0; 8]);
    o.extend(e.chunk_root);
    o.extend(e.content_len.to_le_bytes());
    o
}

pub fn ready_body(r: &Readiness) -> Vec<u8> {
    let mut o = vec![u8::from(r.install_ready)];
    o.extend(r.gates.map(u8::from));
    o.push(r.bond as u8);
    o
}

/// GET_APP's reply: the listing's id text, its measurement, its name, the
/// publisher's name and key, a description, and how many releases.
pub fn app_body(id: &[u8], name: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let mut o = Vec::new();
    lp(&mut o, id);
    o.extend([0; 32]);
    [name, b"A Publisher"].iter().for_each(|f| lp(&mut o, f));
    o.extend(key);
    lp(&mut o, b"What it does.");
    o.extend(1u32.to_le_bytes());
    o
}
