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

//! Snake's home card names a real daily run, a locked rules switch does not
//! move, and the Ranks screen says where the ranks ended up rather than
//! letting a save that failed pass for one that worked.

use crate::snake_state::daily::{pick, DAY_MS};
use crate::snake_state::difficulty::Difficulty;
use crate::snake_state::kept::{line, of_file, of_read, worse, Kept, AMNESIC, DAMAGED, NO_SERVICE};
use crate::snake_state::mode::Mode;
use crate::snake_state::options::Options;

// 2026-10-03 00:00 UTC, in wall-clock milliseconds.
const A_DAY: i64 = 20_729 * DAY_MS;

#[test]
fn the_daily_run_holds_for_the_whole_day() {
    assert!(pick(A_DAY) == pick(A_DAY + DAY_MS - 1));
}

#[test]
fn the_daily_run_changes_with_the_day() {
    assert!(pick(A_DAY) != pick(A_DAY + DAY_MS));
}

#[test]
fn every_mode_and_difficulty_pair_comes_round_in_sixteen_days() {
    let mut seen = Vec::new();
    for day in 0..16 {
        let (mode, level) = pick(A_DAY + day * DAY_MS);
        let key = (mode as u8, level as u8);
        assert!(!seen.contains(&key), "day {day} repeats a pair");
        seen.push(key);
    }
    assert!(pick(A_DAY) == pick(A_DAY + 16 * DAY_MS));
}

#[test]
fn an_unset_clock_still_names_a_run() {
    assert!(pick(-5) == (Mode::Arcade, Difficulty::Easy));
}

#[test]
fn the_wrap_switch_does_not_move_where_the_mode_decides_it() {
    for mode in [Mode::Zen, Mode::Classic] {
        let mut opts = Options::new();
        assert!(!opts.flip(1, mode));
        assert!(!opts.wrap);
    }
}

#[test]
fn the_wrap_switch_moves_where_the_mode_leaves_it_free() {
    let mut opts = Options::new();
    assert!(opts.flip(1, Mode::Arcade));
    assert!(opts.wraps(Mode::Arcade));
}

#[test]
fn obstacles_and_power_ups_move_in_every_mode_and_nothing_else_is_a_switch() {
    let mut opts = Options::new();
    assert!(opts.flip(0, Mode::Zen) && !opts.obstacles);
    assert!(opts.flip(2, Mode::Classic) && !opts.powerups);
    assert!(!opts.flip(3, Mode::Arcade));
}

#[test]
fn ranks_written_and_committed_say_saved_to_disk() {
    assert_eq!(of_file(Ok(()), Ok(())), Kept::Disk);
    assert_eq!(line(Kept::Disk).0, b"Ranks saved to disk");
}

#[test]
fn a_boot_that_keeps_nothing_says_the_ranks_last_until_power_off() {
    let kept = of_file(Ok(()), Err("access denied"));
    assert_eq!(kept, Kept::Session(AMNESIC));
    let (caption, why, warn) = line(kept);
    assert_eq!(caption, b"Ranks kept until power off: ");
    assert_eq!(why, AMNESIC.as_bytes());
    assert!(warn);
}

#[test]
fn a_full_disk_is_named_as_the_reason() {
    assert_eq!(of_file(Ok(()), Err("no space left")), Kept::Session("no space left"));
}

#[test]
fn a_write_that_failed_says_not_saved_and_why() {
    assert_eq!(of_file(Err("vfs ipc failed"), Ok(())), Kept::NotSaved(NO_SERVICE));
    let (caption, why, warn) = line(Kept::NotSaved("access denied"));
    assert_eq!(caption, b"Ranks not saved: ");
    assert_eq!(why, b"access denied");
    assert!(warn);
}

#[test]
fn two_files_are_only_as_kept_as_the_worse_one() {
    let failed = Kept::NotSaved(NO_SERVICE);
    assert_eq!(worse(Kept::Disk, failed), failed);
    assert_eq!(worse(failed, Kept::Disk), failed);
    assert_eq!(worse(Kept::Disk, Kept::Session(AMNESIC)), Kept::Session(AMNESIC));
    assert_eq!(worse(Kept::Quiet, Kept::Disk), Kept::Disk);
}

#[test]
fn a_first_run_with_no_ranks_file_says_nothing() {
    assert_eq!(of_read(Err("vfs open failed"), false), Kept::Quiet);
    assert_eq!(line(Kept::Quiet).0, b"");
}

#[test]
fn stored_ranks_that_did_not_read_are_named_not_shown_as_empty() {
    assert_eq!(of_read(Ok(()), false), Kept::Unread(DAMAGED));
    assert_eq!(of_read(Err("vfs read failed"), false), Kept::Unread("vfs read failed"));
    assert_eq!(of_read(Err("vfs ipc failed"), false), Kept::Unread(NO_SERVICE));
    assert_eq!(line(Kept::Unread(DAMAGED)).0, b"Stored ranks not read: ");
}

#[test]
fn ranks_that_read_cleanly_say_nothing() {
    assert_eq!(of_read(Ok(()), true), Kept::Quiet);
}

/// A button that ends the run and goes to the home screen is called Home on
/// every panel. The play footer and the pause panel called it Quit, and the
/// window stayed open.
#[test]
fn the_button_that_goes_home_is_called_home() {
    let foot = include_str!("../../capsule_snake/src/snake/ui/play_geom_rows.rs");
    let pause = include_str!("../../capsule_snake/src/snake/ui/pause_geom.rs");
    let over = include_str!("../../capsule_snake/src/snake/ui/over_geom.rs");
    let run = include_str!("../../capsule_snake/src/snake/input/click_run.rs");
    assert!(foot.contains("[b\"Pause\", b\"Restart\", b\"Home\"]"));
    assert!(pause.contains("b\"Settings\", b\"Home\"]"));
    assert!(over.contains("b\"Home\"]"));
    for src in [foot, pause, over] {
        assert!(!src.contains("b\"Quit\""), "no button says Quit and stays open");
    }
    assert!(run.contains("2 => nav::go(game, Screen::Home)"), "the footer's third goes home");
    assert!(run.contains("_ => nav::go(game, Screen::Home)"), "the pause panel's last goes home");
}
