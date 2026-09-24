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

//! `install` from the terminal.
//!
//!     install                  list the disks and what each holds
//!     install write <word>     erase the disk whose word that is and install
//!         --yes                skip the typed confirmation (scripts)
//!         --reboot             restart when the read-back passes
//!
//! The word is the last four characters of the disk's serial when the part
//! reports one, the bus name otherwise; `install` prints it beside each disk.

mod cli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = cli::run(&args);
    std::process::exit(code);
}
