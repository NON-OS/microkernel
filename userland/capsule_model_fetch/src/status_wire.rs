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
 * Where a running download stands, as the fetcher answers it on its own
 * endpoint, `tool.model-fetch`, for the store's card while it installs a
 * tier: the installer drains the fetcher's lines and drops them, so the
 * store asks instead. The answer is numbers only: no tier word, no mirror,
 * no file name; the store tells its own tier by the total, the summed
 * length of the tier's pinned files. The fetcher answers this one fixed
 * question and nothing else, so its endpoint takes no command. Pure, so
 * model_fetch_proofs holds the layout both sides read.
 *
 *   0..4    magic "NQF1"
 *   4       stage: FETCHING, CHECKING (the kernel hashing the last bytes),
 *           BROKEN (the connection dropped; it resumes from `done`), or
 *           ANYONE_WAIT (net.anon is building its circuit), or STARTING
 *           (running, nothing to count yet: the route and the size are
 *           still being found)
 *   5       route: 1 Nym, 2 Anyone, 3 Direct
 *   6       BROKEN: the try about to be made, from 2; ANYONE_WAIT: the
 *           step net.anon has reached, 0 before it says; else 0
 *   7       BROKEN: the most tries in a row (get/retry.rs); ANYONE_WAIT:
 *           the steps net.anon counts; else 0
 *   8..16   the tier's bytes in all       16..24  bytes on the volume
 *   24..32  bytes a second, measured, or 0 before enough has come to
 *           say a rate and the time left honestly (`path::eta`)
 *
 * status_write.rs is the fetcher's side, status_read.rs the store's.
 */

/* The question, and the answer's magic. */
pub const ASK: [u8; 4] = *b"NQF?";
pub const MAGIC: [u8; 4] = *b"NQF1";
pub const LEN: usize = 32;

pub const FETCHING: u8 = 1;
pub const CHECKING: u8 = 2;
pub const BROKEN: u8 = 3;
/* Waiting for the Anyone network to build its circuit: bytes 6 and 7 are its step and steps. */
pub const ANYONE_WAIT: u8 = 4;
/* Running, with nothing to count yet. Said rather than left unanswered, so
 * a store asking early learns the fetcher is there and working. */
pub const STARTING: u8 = 5;

pub const NYM: u8 = 1;
pub const ANYONE: u8 = 2;
pub const DIRECT: u8 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Status {
    pub stage: u8,
    pub route: u8,
    pub total: u64,
    pub done: u64,
    pub rate: u64,
    /* While BROKEN: the try about to be made, and of how many. */
    pub try_n: u8,
    pub tries: u8,
}
