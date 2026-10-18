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

//! nox push as its two file readers see it: stand-ins for its args and PUT,
//! and the run and walk modules cut to the file each mounts.

pub mod args {
    // Replaces push/args.rs, whose parser the readers do not call: only the
    // target a one-file push reads.
    pub struct PushArgs {
        pub target: super::super::pull::target::Target,
    }
}
#[path = "push_request.rs"]
pub mod request;
#[path = "push_run.rs"]
mod run;
#[path = "push_walk.rs"]
mod walk;
