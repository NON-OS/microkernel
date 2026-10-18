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

//! The terminal's term module as these readers see it: the real cwd and
//! format_u64, and stand-ins for State and Scrollback.

#[path = "../../../capsule_terminal/src/term/cwd"]
pub mod cwd {
    mod as_bytes;
    mod home;
    mod new;
    mod resolve;
    mod types;

    pub use resolve::resolve;
    pub use types::Cwd;
}

#[path = "../../../capsule_terminal/src/term/util"]
pub mod util {
    mod format_u64;

    pub use format_u64::format_u64;
}

pub mod scrollback {
    // Replaces term/scrollback, which renders each line into styled rows;
    // here a line is kept as it was pushed, with whether it was an error.
    #[derive(Default)]
    pub struct Scrollback {
        pub rows: Vec<(bool, Vec<u8>)>,
    }

    impl Scrollback {
        pub fn push_line(&mut self, line: &[u8]) {
            self.rows.push((false, line.to_vec()));
        }

        pub fn push_error(&mut self, line: &[u8]) {
            self.rows.push((true, line.to_vec()));
        }
    }
}

pub mod state {
    // Replaces term/state: only the fields these readers touch.
    pub struct State {
        pub scrollback: super::scrollback::Scrollback,
        pub cwd: super::cwd::Cwd,
        pub owner_pid: u32,
    }
}
