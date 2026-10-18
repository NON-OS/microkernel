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

//! A9f: the response has one budget, REPLY_MS or the caller's. A peer that
//! sends a byte inside every quiet window is cut off there, REPLY_TOO_SLOW.

use crate::session::response::read_response;
use crate::session::{Io, SessionError, REPLY_MS, REPLY_TOO_SLOW};
use nonos_libc::{mk_uptime_ms, skip_ms};

/// Hands out `body`, `per` bytes a read `gap` ms apart; empty reads take 100.
struct Peer {
    body: Vec<u8>,
    per: usize,
    gap: i64,
}

impl Io for Peer {
    fn write_all(&mut self, _: &[u8]) -> Result<(), SessionError> {
        Ok(())
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        let n = self.body.len().min(self.per).min(into.len());
        into[..n].copy_from_slice(&self.body[..n]);
        self.body.drain(..n);
        skip_ms(if n == 0 { 100 } else { self.gap });
        Ok(n)
    }
}

#[test]
fn a_trickling_peer_is_cut_off_at_the_budget() {
    let mut io = Peer { body: vec![0x17; 1 << 20], per: 1, gap: 3_000 };
    let t0 = mk_uptime_ms();
    let got = read_response(&mut io, Vec::new(), 1 << 20, t0 + REPLY_MS);
    let spent = mk_uptime_ms() - t0;
    assert_eq!(got.err(), Some(REPLY_TOO_SLOW));
    assert!((REPLY_MS..REPLY_MS + 3_000 + 1_000).contains(&spent), "{spent} ms");
    assert!(io.body.len() > (1 << 20) - 200, "about one byte per 3 s was taken");
}

#[test]
fn a_prompt_reply_comes_back_whole() {
    let body: Vec<u8> = (0..40_000u32).map(|i| i as u8).collect();
    let mut io = Peer { body: body.clone(), per: 4096, gap: 5 };
    let got = read_response(&mut io, b"head".to_vec(), 1 << 20, mk_uptime_ms() + REPLY_MS);
    let mut want = b"head".to_vec();
    want.extend_from_slice(&body);
    assert_eq!(got.ok(), Some(want), "everything, behind what came with the flight");
}

#[test]
fn a_callers_shorter_budget_and_a_spent_one_hold() {
    let mut io = Peer { body: vec![1; 1 << 16], per: 1, gap: 1_000 };
    let t0 = mk_uptime_ms();
    let got = read_response(&mut io, Vec::new(), 1 << 20, t0 + 20_000);
    assert_eq!(got.err(), Some(REPLY_TOO_SLOW));
    assert!(mk_uptime_ms() - t0 < 20_000 + 2_000);
    let past = read_response(&mut Peer { body: vec![], per: 1, gap: 0 }, vec![], 16, t0 - 1);
    assert_eq!(past.err(), Some(REPLY_TOO_SLOW), "an end already gone");
}
