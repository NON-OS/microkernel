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

//! Every reader the app store runs over a market reply body: the catalogue
//! list, an app, its ids, a release and its escrow, commerce, the logo and
//! the readiness gates, plus the reply framing in front of them. None of them
//! panics on any bytes. A catalogue list is read one way only: re-encoding
//! the entries gives back exactly the bytes the reader consumed, with each
//! ready byte read as 0 or 1. A logo is accepted only as the writer sends it.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_market_proto::logo::{logo_body, parse_logo};
use nonos_market_proto::*;

fn list(body: &[u8]) {
    let Some(entries) = parse_list(body) else {
        return;
    };
    let mut out = (entries.len() as u32).to_le_bytes().to_vec();
    let mut ready_at = Vec::new();
    for e in &entries {
        out.extend_from_slice(&(e.id.len() as u32).to_le_bytes());
        out.extend_from_slice(&e.id);
        out.extend_from_slice(&e.measurement);
        out.extend_from_slice(&(e.name.len() as u32).to_le_bytes());
        out.extend_from_slice(&e.name);
        ready_at.push(out.len());
        out.push(e.ready as u8);
    }
    assert!(out.len() <= body.len(), "entries longer than the body they came from");
    let mut seen = body[..out.len()].to_vec();
    for &at in &ready_at {
        seen[at] = (seen[at] != 0) as u8;
    }
    assert_eq!(seen, out, "the list reads back to other bytes");
}

fn logo(body: &[u8]) {
    let Some(read) = parse_logo(body) else {
        return;
    };
    let again = logo_body(read.as_ref().map(|l| l.rgba.as_slice()));
    assert_eq!(again.as_deref(), Some(body), "a logo the writer never sends");
}

fn framed(data: &[u8]) {
    if data.len() < 6 {
        return;
    }
    let op = u16::from_le_bytes([data[0], data[1]]);
    let id = u32::from_le_bytes([data[2], data[3], data[4], data[5]]);
    let rx = &data[6..];
    if let Ok(body) = reply_body(rx, op, id) {
        let start = body.as_ptr() as usize - rx.as_ptr() as usize;
        assert!(start == HDR_LEN + STATUS_LEN && start + body.len() <= rx.len());
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, body)) = data.split_first() else {
        return;
    };
    match pick % 9 {
        0 => list(body),
        1 => drop(parse_app(body)),
        2 => drop(parse_app_ids(body)),
        3 => drop(parse_release(body).map(|r| r.short_version().len())),
        4 => drop(parse_release_escrow(body)),
        5 => drop(parse_commerce(body)),
        6 => logo(body),
        7 => drop(parse_readiness(body)),
        _ => framed(body),
    }
});
