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

//! AES-CMAC (RFC 4493), the one MAC every LE Secure Connections function is
//! built from (Vol 3, Part H, 2.2.5). It is the system's one, in nonos_aes,
//! the same the Wi-Fi core signs EAPOL-Key frames with; it takes the
//! message in parts, so the functions pass their fields without first
//! copying them together.

pub use nonos_aes::aes_cmac;
