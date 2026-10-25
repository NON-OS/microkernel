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
//! Usage of the commands that name commands and handle jobs.

pub(crate) const JOBS: &[(&[u8], &[u8], &[u8])] = &[
    (b"type", b"type <name...>", b"say which of the four routes a name runs through"),
    (b"which", b"which <name...>", b"the same as type"),
    (b"history", b"history", b"the commands this session has run"),
    (b"jobs", b"jobs", b"what is running in the background"),
    (b"fg", b"fg <id>", b"bring a job to the foreground"),
    (
        b"bg",
        b"bg <id>",
        b"say that a job runs on; nothing is ever stopped, so every job already runs behind",
    ),
];
