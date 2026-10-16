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

//! The terminal's command module as these readers see it: the real Output, a
//! flags stand-in, and the builtins and dispatch that read whole files.

#[path = "flags.rs"]
pub mod flags;

#[path = "../../../capsule_terminal/src/command/output"]
pub mod output {
    mod new;
    mod types;
    mod writeln;

    pub use types::Output;
}

#[path = "builtin.rs"]
pub mod builtin;
#[path = "dispatch.rs"]
pub mod dispatch;
