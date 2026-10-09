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

//! The wallpaper service's job: the worker's fetch and decode against the
//! real catalog, and the plan the service thread keeps about it. A job
//! starts when the wallpaper wanted is not the one shown and none is out;
//! a job whose wallpaper is no longer wanted is let finish and its picture
//! dropped, and the next starts after it; a job that stops partway keeps
//! what came over and the next try (after the next poll) asks only for the
//! rest; a job that decodes is painted, and nothing starts again for it.
//! The fetch and the decode fit on a worker's stack.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{answer_calls, take_replies};

use crate::catalog_client::proto::{Header, OP_GET_CHUNK, OP_GET_SIZE};
use crate::catalog_client::Download;
use crate::collection::{files, reset, TURN};
use crate::server::serve::serve;
use crate::wallpaper_job::fetch::fetch;
use crate::wallpaper_job::plan::{Outcome, Plan};

const WALLPAPER_PID: u32 = 31;
const CATALOG_PORT: u32 = 4110;
const PEPE: u8 = 62;
/// libc's `WORKER_STACK`: the stack every worker of `WorkerSeat` runs on.
const WORKER_STACK: usize = 64 * 1024;

/// The calls made, as (op, offset). The calls numbered in `lost` (from 0)
/// are served but their replies never come back.
fn link(lost: &[usize]) -> Rc<RefCell<Vec<(u16, u32)>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let seen = Rc::clone(&log);
    let lost = lost.to_vec();
    answer_calls(move |_, req, _| {
        let hdr = Header::decode(req).expect("a whole header");
        let n = {
            let mut calls = seen.borrow_mut();
            calls.push((hdr.op, hdr.offset));
            calls.len() - 1
        };
        let _ = take_replies();
        serve(WALLPAPER_PID, req);
        let (_, reply) = take_replies().remove(0);
        (!lost.contains(&n)).then_some(reply)
    });
    log
}

/// A plan's job outcome with a stand-in picture.
type Job = Outcome<&'static str, u32>;

#[test]
fn a_job_starts_for_the_wallpaper_wanted_and_only_one_at_a_time() {
    let mut plan: Plan<&'static str> = Plan::new();
    assert!(plan.begin().is_none(), "nothing wanted yet");
    plan.want(4);
    assert_eq!(plan.begin(), Some((4, None)));
    assert_eq!(plan.running(), Some(4));
    assert_eq!(plan.begin(), None, "never two at once");
    plan.want(4);
    assert_eq!(plan.begin(), None, "a poll while it runs starts nothing");
}

#[test]
fn a_job_no_longer_wanted_finishes_dropped_and_the_next_starts_after_it() {
    let mut plan: Plan<&'static str> = Plan::new();
    plan.want(4);
    assert_eq!(plan.begin(), Some((4, None)));
    plan.want(7);
    assert_eq!(plan.begin(), None, "4 is still out");
    assert_eq!(plan.finish(4, Job::Decoded(44)), None, "4's picture is dropped");
    assert_eq!(plan.begin(), Some((7, None)), "7 starts at once, no poll needed");
    assert_eq!(plan.finish(7, Job::Decoded(77)), Some(77));
    plan.shown(7);
    assert_eq!(plan.begin(), None);
}

#[test]
fn bytes_of_a_wallpaper_no_longer_wanted_are_let_go() {
    let mut plan: Plan<&'static str> = Plan::new();
    plan.want(4);
    let _ = plan.begin();
    assert_eq!(plan.finish(4, Job::Stopped("4's bytes")), None);
    plan.want(7);
    assert_eq!(plan.begin(), Some((7, None)), "7 starts clean");
    // A stopped job for a wallpaper no longer wanted keeps nothing either.
    plan.want(9);
    assert_eq!(plan.finish(7, Job::Stopped("7's bytes")), None);
    assert_eq!(plan.begin(), Some((9, None)));
}

#[test]
fn a_failure_waits_for_the_next_poll_and_a_worker_not_yet_gone_does_not() {
    let mut plan: Plan<&'static str> = Plan::new();
    plan.want(4);
    let _ = plan.begin();
    assert_eq!(plan.finish(4, Job::Failed("asking the catalog its size")), None);
    assert_eq!(plan.begin(), None, "held until the policy is asked again");
    plan.want(4);
    let (index, kept) = plan.begin().expect("tried again after the poll");
    // The last worker is still on its way out: the very next tick tries.
    plan.not_started(index, kept, false);
    assert_eq!(plan.begin(), Some((4, None)));
    // No worker could be had at all: held, like a failure.
    plan.not_started(4, Some("4's bytes"), true);
    assert_eq!(plan.begin(), None);
    plan.want(4);
    assert_eq!(plan.begin(), Some((4, Some("4's bytes"))), "the bytes came back");
}

#[test]
fn a_picture_that_could_not_be_painted_is_tried_again_after_the_poll() {
    let mut plan: Plan<&'static str> = Plan::new();
    plan.want(4);
    let _ = plan.begin();
    assert_eq!(plan.finish(4, Job::Decoded(44)), Some(44));
    plan.missed(4);
    assert_eq!(plan.begin(), None);
    plan.want(4);
    assert_eq!(plan.begin(), Some((4, None)));
}

#[test]
fn a_failed_job_keeps_its_bytes_and_the_next_one_resumes_then_paints() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let file = &files()[PEPE as usize].1;
    // Call 0 is the size; call 4 is the chunk at 3 * CHUNK_MAX, lost.
    let log = link(&[4]);
    let mut plan: Plan<Download> = Plan::new();
    plan.want(PEPE);
    let (index, kept) = plan.begin().expect("a job for the wallpaper wanted");
    assert!(kept.is_none());
    let first = fetch(CATALOG_PORT, index, kept);
    let Outcome::Stopped(download) = first else { panic!("the job stops at the lost reply") };
    assert_eq!(download.progress(), (3 * 4096, file.len() as u32));
    assert!(plan.finish::<()>(index, Outcome::Stopped(download)).is_none());
    assert!(plan.begin().is_none(), "not again until the next poll");
    let calls_before = log.borrow().len();

    plan.want(PEPE);
    let (index, kept) = plan.begin().expect("the next try");
    let kept = kept.expect("what came over is handed to it");
    let Outcome::Decoded(image) = fetch(CATALOG_PORT, index, Some(kept)) else {
        panic!("the next try fetches the rest and decodes it");
    };
    let calls = log.borrow();
    assert_eq!(calls[calls_before], (OP_GET_CHUNK, 3 * 4096), "not the size, not byte 0");
    assert!(calls[calls_before..].iter().all(|(op, _)| *op != OP_GET_SIZE));
    let image = plan.finish(index, Outcome::Decoded(image)).expect("still wanted: paint it");
    assert_eq!((image.width, image.height), (1920, 1080));
    plan.shown(index);
    plan.want(PEPE);
    assert!(plan.begin().is_none(), "shown: nothing more to fetch");
}

#[test]
fn kept_bytes_of_another_wallpaper_are_not_resumed() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let _log = link(&[2]);
    let Outcome::Stopped(other) = fetch(CATALOG_PORT, 0, None) else {
        panic!("the job for 0 stops at its lost reply");
    };
    let _log = link(&[]);
    let Outcome::Decoded(image) = fetch(CATALOG_PORT, PEPE, Some(other)) else {
        panic!("a fresh fetch of the one asked for");
    };
    assert_eq!(image.pixels.len(), 1920 * 1080);
}

#[test]
fn the_fetch_and_decode_of_the_largest_wallpaper_fit_on_a_worker_stack() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let (largest, _) =
        files().iter().enumerate().max_by_key(|(_, (_, bytes))| bytes.len()).expect("a collection");
    let worker = std::thread::Builder::new()
        .stack_size(WORKER_STACK)
        .spawn(move || {
            let _log = link(&[]);
            match fetch(CATALOG_PORT, largest as u8, None) {
                Outcome::Decoded(image) => Some((image.width, image.height)),
                _ => None,
            }
        })
        .expect("a thread");
    assert_eq!(worker.join().expect("no overflow"), Some((1920, 1080)));
}
