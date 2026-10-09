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

//! The wallpaper service fetching wallpapers from the catalog, both sides
//! real: its client's calls are served by the catalog's `serve` over the
//! collection the real files make, and a call can be lost the way one past
//! its budget is: the catalog still serves it, and the late reply never
//! reaches the client. Every wallpaper comes over whole and decodes at full
//! size through the service's own decoder; a try that loses a reply keeps
//! what came before it and the next try asks for the rest; and the first
//! call of a try has the budget for a read from the store whatever its
//! offset, since the catalog may have let the wallpaper go.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{answer_calls, take_replies};

use crate::collection::{files, flip, reset, TURN};
use crate::server::serve::serve;
use crate::wallpaper_client::budget::{FIRST_CHUNK_MS, NEXT_CHUNK_MS, SIZE_MS};
use crate::wallpaper_client::proto::{Header, CHUNK_MAX, OP_GET_CHUNK, OP_GET_SIZE};
use crate::wallpaper_client::Download;
use crate::wallpaper_decode::decode_jpeg;

const WALLPAPER_PID: u32 = 31;
const CATALOG_PORT: u32 = 4110;
const PEPE: u8 = 62;

/// One call the wallpaper service made: its op, the offset it asked for,
/// and the budget it gave the call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Call {
    op: u16,
    offset: u32,
    budget: u64,
}

/// Every call the service makes from here on goes to the catalog's `serve`;
/// the calls numbered in `lost` (from 0) are served but their replies never
/// come back. The calls made are logged.
fn link(lost: &[usize]) -> Rc<RefCell<Vec<Call>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let seen = Rc::clone(&log);
    let lost = lost.to_vec();
    answer_calls(move |port, req, budget| {
        assert_eq!(port, CATALOG_PORT as u64, "the service calls the catalog");
        let hdr = Header::decode(req).expect("a whole header");
        let n = {
            let mut calls = seen.borrow_mut();
            calls.push(Call { op: hdr.op, offset: hdr.offset, budget });
            calls.len() - 1
        };
        let _ = take_replies();
        serve(WALLPAPER_PID, req);
        let mut replies = take_replies();
        assert_eq!(replies.len(), 1, "every call is answered once");
        let (to, reply) = replies.remove(0);
        assert_eq!(to, WALLPAPER_PID);
        (!lost.contains(&n)).then_some(reply)
    });
    log
}

fn chunks(len: usize) -> usize {
    len.div_ceil(CHUNK_MAX)
}

#[test]
fn every_wallpaper_comes_over_whole_and_decodes_at_full_size() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    for (i, (slug, file)) in files().iter().enumerate() {
        let log = link(&[]);
        let mut d = Download::start(CATALOG_PORT, i as u8).expect(slug);
        assert!(d.resume(CATALOG_PORT), "{slug} in one try");
        let bytes = d.into_bytes();
        assert!(bytes == *file, "{slug} came over whole");
        assert_eq!(
            log.borrow().len(),
            1 + chunks(file.len()),
            "{slug}: the size, then each chunk once"
        );
        let img = decode_jpeg(&bytes).unwrap_or_else(|| panic!("{slug} decodes"));
        assert_eq!((img.width, img.height), (1920, 1080), "{slug}");
        assert_eq!(img.pixels.len(), 1920 * 1080, "{slug}");
    }
}

#[test]
fn a_try_that_loses_a_reply_goes_on_from_where_it_stopped() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let file = &files()[PEPE as usize].1;
    // Call 0 is the size and call 1 the first chunk; call 6 is the chunk at
    // 5 * CHUNK_MAX, whose reply comes too late.
    let log = link(&[6]);
    let mut d = Download::start(CATALOG_PORT, PEPE).unwrap();
    assert!(!d.resume(CATALOG_PORT), "the try ends at the lost reply");
    let first_try = log.borrow().len();
    assert_eq!(first_try, 7);
    assert!(d.resume(CATALOG_PORT), "the next try finishes it");
    assert!(d.into_bytes() == *file, "whole, every byte in its place");
    let calls = log.borrow();
    let next: Vec<Call> = calls[first_try..].to_vec();
    // Not the size, and not byte 0 again: the chunk the lost reply was for.
    assert_eq!(
        next[0],
        Call { op: OP_GET_CHUNK, offset: 5 * CHUNK_MAX as u32, budget: FIRST_CHUNK_MS }
    );
    assert!(next[1..].iter().all(|c| c.op == OP_GET_CHUNK && c.budget == NEXT_CHUNK_MS));
    assert_eq!(calls.len(), 1 + chunks(file.len()) + 1, "only the lost chunk is asked twice");
    assert_eq!(calls[0], Call { op: OP_GET_SIZE, offset: 0, budget: SIZE_MS });
}

#[test]
fn the_first_call_of_a_try_has_time_for_a_read_whatever_its_offset() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let file = &files()[PEPE as usize].1;
    let last_call = chunks(file.len());
    // The last chunk's reply is lost; the catalog served it and so let the
    // wallpaper go.
    let log = link(&[last_call]);
    let mut d = Download::start(CATALOG_PORT, PEPE).unwrap();
    assert!(!d.resume(CATALOG_PORT));
    // The catalog no longer holds it: with a byte of it changed on the
    // device, the next ask of that last chunk reads the store and is refused.
    let at: usize = files()[..PEPE as usize].iter().map(|(_, b)| b.len()).sum::<usize>() + 3;
    flip(at);
    assert!(!d.resume(CATALOG_PORT), "a read from the store, at a non-zero offset");
    reset();
    assert!(d.resume(CATALOG_PORT));
    assert!(d.into_bytes() == *file);
    let calls = log.borrow();
    let tail = &calls[calls.len() - 2..];
    let last_offset = ((file.len() - 1) / CHUNK_MAX * CHUNK_MAX) as u32;
    for c in tail {
        assert_eq!(*c, Call { op: OP_GET_CHUNK, offset: last_offset, budget: FIRST_CHUNK_MS });
    }
}
