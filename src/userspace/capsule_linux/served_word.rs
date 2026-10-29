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

//! The word a served launch gave its run (syscall/microkernel/
//! app_launch_word.rs), kept with the package it was given for until the
//! next launch, so the run carries it in either role: its first start, and
//! its start again with the network.

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

static WORD: Mutex<Option<(String, String)>> = Mutex::new(None);

/// The latest launch: `package`, and the `word` it gave, if any.
pub(super) fn keep(package: &str, word: Option<&str>) {
    *WORD.lock() = word.map(|w| (String::from(package), String::from(w)));
}

/// What a run of `package` is spawned with: `run`, its name, then `word`
/// and the word its launch gave, if it gave one.
pub(super) fn run_words(package: &str) -> Vec<String> {
    let mut words = alloc::vec![String::from("run"), String::from(package)];
    if let Some((_, w)) = WORD.lock().as_ref().filter(|(p, _)| p == package) {
        words.extend([String::from("word"), w.clone()]);
    }
    words
}
