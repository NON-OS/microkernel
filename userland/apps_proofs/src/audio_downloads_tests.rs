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

//! Music's Downloads page: one download runs at a time, each row offers what
//! can be done with it, cancel and resume keep what came, and every row says
//! where it stands in words.

use crate::audio_downloads::list::{Act, Downloads, State, MAX_ROWS};
use crate::audio_downloads::row_text::{label, left, permille, rate, size, status};
use crate::audio_rungs::rungs;

const WEB_PAGE: &str = "That address gave a web page, not an MP3";
const DROPPED: &str = "The connection kept dropping";

#[test]
fn one_runs_at_a_time_in_the_order_asked() {
    let mut d = Downloads::new();
    let a = d.add("https://h/a.mp3", "a").unwrap();
    let b = d.add("https://h/b.mp3", "b").unwrap();
    assert_eq!(d.add("https://h/a.mp3", "a"), None, "the same link twice is one download");
    assert_eq!(d.start_next(0).map(|r| r.id), Some(a));
    assert!(d.start_next(0).is_none(), "the second waits");
    assert_eq!(d.active(), Some(a));
    d.ended(a, Ok("/home/nonos/music/a.mp3".into()));
    assert_eq!(d.start_next(10).map(|r| r.id), Some(b));
    assert_eq!(d.get(a).unwrap().state, State::Done("/home/nonos/music/a.mp3".into()));
}

#[test]
fn cancel_keeps_what_came_and_resume_goes_on() {
    let mut d = Downloads::new();
    let a = d.add("https://h/a.mp3", "a").unwrap();
    let queued = d.add("https://h/q.mp3", "q").unwrap();
    d.start_next(0);
    d.progress(a, 3_000_000, Some(8_000_000), 0);
    assert!(d.cancel(a), "a running download is the worker's to stop");
    assert_eq!(d.get(a).unwrap().state, State::Stopping);
    assert!(d.get(a).unwrap().acts().is_empty(), "nothing to press while it stops");
    // Whatever the worker says last, the person cancelled it.
    d.ended(a, Err((DROPPED, true)));
    assert_eq!(d.get(a).unwrap().state, State::Cancelled);
    assert_eq!(d.get(a).unwrap().acts(), &[Act::Resume, Act::Remove]);
    assert!(!d.cancel(queued), "a waiting one is cancelled here");
    assert_eq!(d.get(queued).unwrap().state, State::Cancelled);
    assert!(d.resume(a));
    assert_eq!(d.get(a).unwrap().state, State::Queued);
    assert_eq!(d.get(a).unwrap().done, 3_000_000, "it goes on from what came");
    assert!(status(d.get(a).unwrap()).contains("2.8 MB"));
}

#[test]
fn a_file_that_is_not_audio_is_not_offered_again() {
    let mut d = Downloads::new();
    let a = d.add("https://h/page", "page").unwrap();
    d.start_next(0);
    d.ended(a, Err((WEB_PAGE, false)));
    assert_eq!(d.get(a).unwrap().acts(), &[Act::Remove]);
    assert!(!d.resume(a));
    assert_eq!(status(d.get(a).unwrap()), WEB_PAGE);
    let b = d.add("https://h/b.mp3", "b").unwrap();
    d.start_next(0);
    d.ended(b, Err((DROPPED, true)));
    assert_eq!(d.get(b).unwrap().acts(), &[Act::Resume, Act::Remove]);
}

#[test]
fn speed_and_time_left_are_measured_over_a_second() {
    let mut d = Downloads::new();
    let a = d.add("https://h/a.mp3", "a").unwrap();
    d.start_next(0);
    d.progress(a, 500_000, Some(10_000_000), 400);
    assert_eq!(d.get(a).unwrap().speed, 0, "not yet a second");
    d.progress(a, 1_000_000, Some(10_000_000), 1_000);
    assert_eq!(d.get(a).unwrap().speed, 1_000_000);
    assert_eq!(d.get(a).unwrap().eta_secs(), Some(9));
    d.progress(a, 1_200_000, None, 2_000);
    assert_eq!(d.get(a).unwrap().speed, (1_000_000 * 3 + 200_000) / 4, "smoothed");
    assert_eq!(d.get(a).unwrap().total, Some(10_000_000), "a length once said is kept");
    // The server sent the file again from its start: no negative speed.
    d.progress(a, 10, None, 3_000);
    assert_eq!(d.get(a).unwrap().done, 10);
    let line = status(d.get(a).unwrap());
    assert!(line.starts_with("10 B of 9.5 MB (0%)"), "{line}");
    assert!(line.contains("/s"), "{line}");
}

#[test]
fn finished_rows_clear_and_the_list_stays_bounded() {
    let mut d = Downloads::new();
    for i in 0..MAX_ROWS {
        let id = d.add(&format!("https://h/{i}.mp3"), "x").unwrap();
        d.start_next(0);
        d.ended(id, Ok(format!("/m/{i}.mp3")));
    }
    let more = d.add("https://h/more.mp3", "more").unwrap();
    assert_eq!(d.rows().len(), MAX_ROWS, "the oldest finished made room");
    assert!(!d.remove(more), "a waiting one is cancelled first");
    d.clear_finished();
    assert_eq!(d.rows().len(), 1);
    assert_eq!(d.rows()[0].id, more);
}

#[test]
fn rows_say_where_they_stand() {
    let mut d = Downloads::new();
    let a = d.add("https://h/a.mp3", "a").unwrap();
    assert_eq!(status(d.get(a).unwrap()), "Waiting for the download before it");
    d.start_next(0);
    assert_eq!(status(d.get(a).unwrap()), "Connecting");
    assert_eq!(permille(d.get(a).unwrap()), None);
    d.progress(a, 2_500_000, Some(10_000_000), 0);
    assert_eq!(permille(d.get(a).unwrap()), Some(250));
    assert_eq!(label(Act::Resume), "Resume");
    assert_eq!(size(1023), "1023 B");
    assert_eq!(size(1536 * 1024), "1.5 MB");
    assert_eq!(rate(2048), "2 KB/s");
    assert_eq!(left(42), "42 s left");
    assert_eq!(left(125), "2 min 5 s left");
    assert_eq!(left(7_260), "2 h 1 min left");
}

#[test]
fn the_grid_motif_ends_on_any_cover() {
    // A queue thumbnail: 3% of its side is 0, which never grew before.
    for side in 0..80 {
        let lines = rungs(10, 10 + side, side);
        assert!(lines.len() <= side as usize);
        assert!(lines.windows(2).all(|w| w[0] < w[1]));
        assert!(lines.iter().all(|&y| y > 10 && y < 10 + side));
    }
    assert!(rungs(0, i32::MAX, 1000).len() < 64, "it ends even at the far edge");
}
