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

//! The endpoints the personality answers on in each role it is spawned for.

/// Each role is its own live process with its own endpoints: two of them
/// announcing one endpoint is a race over which answers.
pub(super) struct Role {
    pub name: &'static str,
    pub port: u32,
    pub inbox: &'static str,
    pub reply_port: u32,
    pub tag: &'static [u8],
}

pub(super) const INSTALL: Role = Role {
    name: "app.linux.install",
    port: 4938,
    inbox: "endpoint.app.linux.install.reply",
    reply_port: 4939,
    tag: b"[LINUX-INSTALL] elf error:",
};

pub(super) const RUN: Role = Role {
    name: "app.linux.run",
    port: 4942,
    inbox: "endpoint.app.linux.run.reply",
    reply_port: 4943,
    tag: b"[LINUX-RUN] elf error:",
};
