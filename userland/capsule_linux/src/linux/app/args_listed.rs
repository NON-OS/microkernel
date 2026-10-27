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

//! A market program's arguments after argv[0]: its listed arguments first,
//! each but those whose option the person names themself, then the person's
//! own words as given. Only the person's words before a "--" are read as
//! options; nothing is split, joined or read by a shell.

use alloc::vec::Vec;

use nonos_linux_policy::args::{names, words};

/// `listed` is a block nonos_linux_policy::args::check has accepted, or empty.
pub fn with_listed(listed: &[u8], person: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let options = || person.iter().take_while(|w| w.as_slice() != b"--");
    let kept = words(listed).filter(|w| !options().any(|given| names(given, w)));
    let mut argv: Vec<Vec<u8>> = kept.map(<[u8]>::to_vec).collect();
    argv.extend(person);
    argv
}
