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

//! Prove signal delivery end to end: a handler installed, a signal raised at
//! this thread, the handler run and returned from. Exits 0 only on delivery.

use std::process::ExitCode;

use nonos_linux_guests::signal_probe;

fn main() -> ExitCode {
    match signal_probe::scan() {
        true => ExitCode::SUCCESS,
        false => ExitCode::FAILURE,
    }
}
