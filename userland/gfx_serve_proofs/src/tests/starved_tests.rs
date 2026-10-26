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
//! A starved owner is not a dead one: a compositor the kernel shows alive
//! keeps its frame while unheard, up to OWNER_HUNG_MS, and is blanked then
//! as hung. One gone is blanked at OWNER_SILENCE_MS. Another pid alive does
//! not count for the owner, nor does a row on the owner's pid
//! younger than the silence: the pid was reused after the owner died.

use super::silence_tests::shown;
use crate::model::wire::COMPOSITOR;
use crate::watch::OWNER_SILENCE_MS;
use crate::watch_starved::OWNER_HUNG_MS;

#[test]
fn a_live_owner_keeps_its_frame_until_it_is_taken_as_hung() {
    let (mut s, _) = shown();
    s.platform.alive.push((COMPOSITOR, 60_000));
    for t in [OWNER_SILENCE_MS, 5_000, OWNER_HUNG_MS - 1] {
        assert!(!s.watch(t), "alive at {t} ms: starved, not gone");
        assert!(s.display.fb[..1024 * 768].iter().all(|&p| p == 0x5A5A_5A5A), "{t}");
    }
    assert!(!s.take_tell(), "nothing cleared, nothing said");
    assert!(s.watch(OWNER_HUNG_MS), "still unheard at the cap: hung");
    assert!(s.display.fb[..1024 * 768].iter().all(|&p| p == 0));
}

#[test]
fn an_owner_gone_is_blanked_at_the_silence() {
    let (mut s, _) = shown();
    s.platform.alive.push((COMPOSITOR + 1, 60_000));
    assert!(s.watch(OWNER_SILENCE_MS), "another pid alive is not the owner");
}

#[test]
fn a_live_stranger_on_a_dead_owner_s_pid_is_not_the_owner() {
    let (mut s, _) = shown();
    // Alive 1 s at a 3 s silence: started after the owner was last heard.
    s.platform.alive.push((COMPOSITOR, 1_000));
    assert!(s.watch(OWNER_SILENCE_MS), "a reused pid counts as gone");
}

#[test]
fn a_starved_owner_heard_again_is_watched_afresh() {
    let (mut s, _) = shown();
    s.platform.alive.push((COMPOSITOR, 60_000));
    assert!(!s.watch(OWNER_HUNG_MS - 1));
    s.heard(COMPOSITOR, OWNER_HUNG_MS - 1);
    s.platform.alive.clear();
    let again = OWNER_HUNG_MS - 1 + OWNER_SILENCE_MS;
    assert!(!s.watch(again - 1) && s.watch(again), "gone now: the silence counts");
}
