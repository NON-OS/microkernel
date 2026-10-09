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

//! Driving the loader to its end as the seeder's idle slots do, and the
//! turn proofs take at the status word, which is one per process.

use std::sync::{Mutex, MutexGuard};

use super::blk::error::BlkError;
use super::blk::load::{Load, Step};

/// The files a load staged: each name with its bytes, in table order.
pub type Staged = Vec<(String, Vec<u8>)>;

/// Every file the loader staged, in table order, or why it stopped.
pub fn load() -> Result<Staged, BlkError> {
    load_counting().map(|(files, _)| files)
}

/// The staged files and the count of damaged entries left out.
pub fn load_counting() -> Result<(Staged, usize), BlkError> {
    let mut load = Load::begin()?;
    loop {
        match load.step_for(8) {
            Step::More => {}
            Step::Done(staged, refused) => {
                return Ok((staged.into_iter().map(|e| (e.name, e.data)).collect(), refused));
            }
            Step::Failed(e) => return Err(e),
        }
    }
}

/// The errno a store op answers for `e`, through the handlers' own map.
pub fn errno(e: BlkError) -> i32 {
    crate::map_blk_err(e)
}

/// Files as name, length and digest, so a failing comparison prints a line
/// per file rather than every byte of them.
pub fn digests(files: &[(String, Vec<u8>)]) -> Vec<(String, usize, [u8; 16])> {
    files.iter().map(|(n, d)| (n.clone(), d.len(), nonos_disk_map::digest16(d))).collect()
}

/// What a boot finds after `op` is cut short at every sector it writes.
///
/// `op` runs once on the disk as it stands, to count the sectors it puts
/// down; then, from the same disk each time, once per count with the power
/// failing after that many sectors, and once more uncut. `check` gets the
/// cut and the next boot's load: its files as `digests` give them and the
/// count of entries it left out as damaged.
pub fn every_cut(op: &dyn Fn(), check: &dyn Fn(usize, Result<Loaded, BlkError>)) {
    let before = nonos_libc::disk::image();
    op();
    let sectors = nonos_libc::disk::landed();
    assert!(sectors > 0, "the op wrote nothing to cut");
    for cut in 0..=sectors {
        nonos_libc::disk::restore(&before);
        nonos_libc::disk::power_fails_after(cut);
        op();
        nonos_libc::disk::power_on();
        check(cut, load_counting().map(|(files, refused)| (digests(&files), refused)));
    }
}

pub type Loaded = (Vec<(String, usize, [u8; 16])>, usize);

static STATUS: Mutex<()> = Mutex::new(());

/// Held while a proof sets and reads the status word.
pub fn status_turn() -> MutexGuard<'static, ()> {
    STATUS.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The codes the desktop shell calls a damaged or unreadable store, read from
/// its own source (`capsule_desktop_shell/src/state/store_word.rs`), so a
/// proof about which side of that line a code falls on is about the shell's
/// line. The shell also says a line for no disk (1), a disk that did not
/// answer (2) and one that refused reads (7); those are about the device, not
/// the store's contents, so only the arms that name damage or a store that
/// could not be read count here.
pub fn shell_corrupt_codes() -> Vec<u32> {
    let src = include_str!("../../../capsule_desktop_shell/src/state/store_word.rs");
    let codes: Vec<u32> = src
        .lines()
        .filter(|l| l.contains("damaged") || l.contains("could not be read"))
        .filter_map(|l| l.trim().split_once("=> Some"))
        .flat_map(|(arm, _)| arm.split('|').map(|c| c.trim().parse::<u32>().expect("a code")))
        .collect();
    assert!(!codes.is_empty());
    codes
}
