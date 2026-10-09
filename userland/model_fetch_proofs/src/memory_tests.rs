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
 * The memory check before a download, on the memory the signed catalogue
 * gives each tier, by need.rs's rule, the one setup and the store hold a
 * tier to: on an installed NONOS the run must leave 1 GiB for the system;
 * on a live boot the file is held in memory beside the run, under what the
 * kernel keeps free. A tier that does not fit is refused with one sentence
 * naming what it needs and what there is, before a byte is fetched; memory
 * the kernel would not report refuses nothing; and the installer reads the
 * refusal as its own reason, not a download that broke.
 */

use crate::catalogue::parse::parse;
use crate::install::fetch_exit::TOO_LITTLE_MEMORY;
use crate::install::model_dep::fetched;
use crate::install::why::Why;
use crate::memory_need::too_large;
use crate::need::Room;
use crate::tests::written;

const GB: u64 = 1_000_000_000;
const GIB: u64 = 1 << 30;

pub fn need(word: &str) -> u64 {
    let blob = written();
    let cat = parse(&blob[..blob.len() - 64]).unwrap();
    cat.tier(word.as_bytes()).unwrap().memory
}

pub fn weights(word: &str) -> u64 {
    crate::pins::all().filter(|p| p.tier == word).map(|p| p.bytes).sum()
}

fn refused(word: &str, machine: u64, room: Room) -> Option<String> {
    too_large(word, weights(word), need(word), Some(machine), room)
}

#[test]
fn the_stick_tier_runs_on_an_installed_two_gigabyte_machine_and_4b_does_not() {
    for word in ["small", "qwen3-0.6b"] {
        assert!(need(word) < GB, "{word} needs {}", need(word));
        assert_eq!(refused(word, 2 * GIB, Room::Disk), None, "{word}");
    }
    let four = need("qwen3-4b");
    assert!(four > 2 * GB && four < 4 * GB, "qwen3-4b needs {four}");
    assert!(refused("qwen3-4b", 2 * GIB, Room::Disk).is_some());
    assert!(refused("qwen3-4b", 3 * GIB, Room::Disk).is_some());
    assert_eq!(refused("qwen3-4b", 4 * GIB, Room::Disk), None);
}

/* A live boot holds the file too: 0.6B needs 3 GiB there, 4B 8 GiB. */
#[test]
fn a_live_boot_holds_the_file_beside_the_run() {
    assert!(refused("qwen3-0.6b", 2 * GIB, Room::Memory).is_some());
    assert_eq!(refused("qwen3-0.6b", 3 * GIB, Room::Memory), None);
    assert!(refused("qwen3-4b", 6 * GIB, Room::Memory).is_some());
    assert_eq!(refused("qwen3-4b", 8 * GIB, Room::Memory), None);
    let line = refused("qwen3-4b", 6 * GIB, Room::Memory).unwrap();
    assert!(line.contains("on this live boot its 2.49 GB file is held in memory too"), "{line}");
    assert!(line.contains("1.61 GB of it kept for the system"), "{line}");
}

#[test]
fn the_refusal_names_the_tier_what_it_needs_and_what_there_is() {
    let line = too_large("qwen3-32b", 19_000_000_000, 21_500_000_000, Some(8_000_000_000), Room::Disk)
        .unwrap();
    assert!(line.starts_with("qwen3-32b needs 21.50 GB of memory to run;"), "{line}");
    assert!(line.contains("this machine has 8.00 GB in all, 1.07 GB of it kept"), "{line}");
    assert!(line.contains("not downloaded") && line.contains("qwen tiers"), "{line}");
}

#[test]
fn exactly_enough_fits_and_unknown_memory_refuses_nothing() {
    assert_eq!(too_large("small", 1, GB, Some(GB + GIB), Room::Disk), None);
    assert!(too_large("small", 1, GB + 1, Some(GB + GIB), Room::Disk).is_some());
    assert_eq!(too_large("max", u64::MAX, u64::MAX, None, Room::Memory), None);
}

#[test]
fn the_installer_reads_it_as_too_little_memory() {
    assert_eq!(fetched(i64::from(TOO_LITTLE_MEMORY)), Err(Why::ModelTooLarge));
    assert_eq!(Why::ModelTooLarge.code(), 19);
}

/* The Terminal's `help qwen`, as it ships. */
#[path = "../../capsule_terminal/src/command/builtin/qwen/help.rs"]
mod qwen_help;

/*
 * What `help qwen` says a tier needs beyond its file is what need.rs, the
 * rule the fetcher and the Store hold a download to, adds for the tier that
 * needs most: never a figure larger or smaller than the one in force.
 */
#[test]
fn help_says_what_a_tier_needs_beyond_its_file() {
    let pinned = |tier: &str| -> u64 {
        crate::pins::all().filter(|p| p.tier == tier).map(|p| p.bytes).sum()
    };
    let beyond = |s: &(&str, u64, u64, u64, u64)| {
        crate::need::memory(s.0, pinned(s.0)).unwrap() - pinned(s.0)
    };
    let most = crate::need::SHAPES.iter().map(beyond).max().unwrap();
    assert!(most <= 800_000_000 && most > 700_000_000, "the most beyond a file is {most}");
    let help: Vec<String> =
        qwen_help::HELP.iter().map(|l| String::from_utf8_lossy(l).into_owned()).collect();
    assert!(help.iter().any(|l| l.contains("plus up to 0.8 GB")), "{help:?}");
    assert!(!help.iter().any(|l| l.contains("1-2 GB")), "{help:?}");
}
