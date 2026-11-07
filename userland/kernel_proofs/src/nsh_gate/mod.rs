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

/*
 * The nsh/1 host's gate: a release or production kernel carries nsh.front
 * and nsh.attest only through nonos-server, the server profile's marker,
 * nsh.front never without nsh.attest, and nsh.attest asks the kernel for
 * every capability its Capsule.mk declares and never for Network. Read
 * where they live: the kernel's compile_error! gates, the features, the
 * profiles and the spawn.
 */

#[cfg(test)]
mod tests;
