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

//! The nox builtins that read a whole file: read, copy and push.

#[path = "../../../capsule_terminal/src/command/builtin/nox/copy.rs"]
pub mod copy;
#[path = "../../../capsule_terminal/src/command/builtin/nox/ensure_pid.rs"]
pub mod ensure_pid;
#[path = "../../../capsule_terminal/src/command/builtin/nox/read.rs"]
pub mod read;

pub mod pull {
    pub mod target {
        // Replaces pull/target.rs, whose parser the push readers do not
        // call: the host, port and path a PUT is sent to.
        pub struct Target {
            pub host: Vec<u8>,
            pub port: u16,
            pub path: Vec<u8>,
        }
    }
}

#[path = "push.rs"]
pub mod push;

#[path = "nox_tests.rs"]
mod nox_tests;
