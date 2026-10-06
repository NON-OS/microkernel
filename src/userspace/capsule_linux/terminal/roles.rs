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

//! The roles a terminal's runs are spawned in.

use super::super::roles::Role;

/// A terminal's own run of a Qwen tier, one role per slot. Each has the
/// endpoints the manifest declares for it, so two terminals each hold a run
/// and neither collides with the store's `app.linux.run`. Nothing beyond
/// LINUX_CAPS: the run reads its model and talks to its terminal, no more.
pub(super) const TERMINAL: [Role; 2] = [
    Role {
        name: "app.linux.term.1",
        port: 4946,
        inbox: "endpoint.app.linux.term.1.reply",
        reply_port: 4947,
        tag: b"[LINUX-TERM] elf error:",
        extra_caps: 0,
    },
    Role {
        name: "app.linux.term.2",
        port: 4948,
        inbox: "endpoint.app.linux.term.2.reply",
        reply_port: 4949,
        tag: b"[LINUX-TERM] elf error:",
        extra_caps: 0,
    },
];
