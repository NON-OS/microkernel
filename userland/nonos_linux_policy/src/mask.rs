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

//! What capsule_linux serves a program, one bit per service. A bit not
//! named here means nothing yet, so a policy carrying one is refused rather
//! than read as a grant a later build might honour.

/// Its own windows on the display, and the input routed to them.
pub const WAYLAND: u64 = 1 << 0;
/// Outbound streams through nonos_route_link only; Direct is never a route.
pub const NET_ANON: u64 = 1 << 1;
/// Listening on loopback, inside its own family.
pub const NET_LISTEN_LO: u64 = 1 << 2;
/// Its own /home/<app>.
pub const FILES_HOME: u64 = 1 << 3;
/// The person's files.
pub const FILES_USER: u64 = 1 << 4;
/// Reading the models.
pub const MODELS_RO: u64 = 1 << 5;
/// Executing other proved programs.
pub const EXEC: u64 = 1 << 6;
/// Anonymous memory made executable.
pub const JIT: u64 = 1 << 7;
/// A terminal.
pub const TTY: u64 = 1 << 8;

/// Every bit with a meaning.
pub const KNOWN: u64 =
    WAYLAND | NET_ANON | NET_LISTEN_LO | FILES_HOME | FILES_USER | MODELS_RO | EXEC | JIT | TTY;

/// Whether `mask` holds every bit of `want`.
pub const fn grants(mask: u64, want: u64) -> bool {
    mask & want == want
}

/// What a program runs with after an exec: never more than the program
/// that exec'd it held, so a chain of execs cannot widen the box.
pub const fn narrow(parent: u64, program: u64) -> u64 {
    parent & program
}
