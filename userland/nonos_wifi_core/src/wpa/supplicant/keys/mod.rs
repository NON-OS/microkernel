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

//! Recover the group keys from an authentic message 3 or group message 1:
//! AES-key-unwrap the key data under the KEK, read its KDEs, and accept a GTK
//! only if it is a CCMP-128 key (16 bytes) and an IGTK only if it is a
//! BIP-CMAC-128 key (16 bytes). The unwrapped plaintext is wiped before return.

mod parse;
mod pick;
mod store;

pub(super) use parse::{parse_group_keys, rsc_pn, wipe};
pub(super) use store::UNWRAPPED_MAX;
