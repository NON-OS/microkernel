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

//! The station's addresses, each a locally administered individual one
//! (IEEE Std 802: the first octet's two low bits). Each network gets its
//! own, derived with a keyed hash from a per-boot secret and the network's
//! identity: a reconnect to the same network this boot keeps it, and two
//! networks see unrelated ones, so they cannot link this machine between
//! them. A scan that transmits goes out from an address of its own, fresh
//! each time. Nothing of the secret outlives the boot
//! (`Addresses::for_this_boot`).

mod bits;
mod keys;
mod kind;
mod network;
mod scan;
pub use bits::{
    identity, local_unicast, security_of, SECURITY_OPEN, SECURITY_PASSPHRASE, SECURITY_WPA3,
};
pub use keys::Addresses;
pub use kind::Security;
pub use network::{one_off, KEY_LEN};
