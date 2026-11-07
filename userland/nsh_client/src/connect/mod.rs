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

//! `nsh`, the client's command line: connect to a host, check it as the
//! client checks every host (its hybrid signature, its `NONOSATT` document
//! for this handshake against the fleet's releases, its enrolment under the
//! fleet's authority, its firmware), and only then log in with the user's
//! key and grant. Exit status: the shell's, or 255 when the session was
//! refused or broken, or 2 for a usage error.

pub mod args;
mod dial;
pub mod explain;
mod explain_host;
mod explain_reason;
mod files;
mod hex;
mod input;
mod load;
#[cfg(feature = "nonos")]
pub mod probe;
#[cfg(feature = "nonos")]
mod probe_keep;
#[cfg(feature = "nonos")]
pub mod probe_show;
mod releases;
mod report;
mod report_record;
mod session;
mod session_keys;
mod start;
pub mod term;

pub use input::{keys, urandom};

use alloc::string::String;
use alloc::vec::Vec;
use report::say;
use std::io::Write;
use std::sync::mpsc::Receiver;

/// Run `nsh` with `args`, typed keys from `keys`, and the shell's output
/// to `out`; the exit status.
pub fn run(argv: &[String], keys: Receiver<Vec<u8>>, out: &mut dyn Write) -> i32 {
    let Some(a) = args::parse(argv) else {
        say(args::USAGE);
        return 2;
    };
    match dial::connect(&a, keys, out) {
        Ok(status) => status.clamp(0, 255),
        Err(why) => {
            say(&why);
            255
        }
    }
}
