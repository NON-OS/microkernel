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

mod handlers;
pub mod outputs;
mod respond;
mod runner;

pub use runner::run;

use nonos_libc::{mk_munmap, mk_pid_alive};

use outputs::Output;

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Let a held output go: unmapping it gives its memory back, and its
/// surface slot once no client maps it.
fn release_output(out: Output) {
    let _ = mk_munmap(out.base as *mut u8, out.len);
}
