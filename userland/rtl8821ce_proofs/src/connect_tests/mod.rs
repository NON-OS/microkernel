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

//! Proofs for the connect request the serve stage parses and the status codes
//! a join reports back. The request body comes over IPC from any client, so
//! every length in it is checked before use; the flags octet is optional so a
//! client that predates it still connects. Every way a join can end maps to a
//! code the client can name, with the WPA3 outcomes (a downgrade refused, a
//! wrong SAE password, a message 3 that did not match the beacon) kept apart
//! from a plain timeout. Before a join the hunt transmits nothing for a
//! network that broadcasts its name; only a hidden one is probed, by name.

mod codes;
mod probe;
mod request;
