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

//! The receive half of an associated link: what the access point sent, checked
//! before anything reaches the IP stack or the supplicant.
//!
//! A data frame is taken only from the BSS (FromDS, transmitter the BSSID) to
//! this station or a group address. Fragments and A-MSDUs are refused (this
//! station negotiates neither, and both are what the FragAttacks injections
//! ride on). On a protected link an unprotected frame is refused unless it is
//! EAPOL (a repeated message 3 arrives in the clear before the AP installs the
//! pairwise key); every protected frame is decrypted, by the chip (which
//! leaves the CCMP header and MIC in place) or here under the pairwise or
//! group key its key index names, and must pass the per-TID replay check.
//! EAPOL goes to the supplicant, which answers the group key handshake; the
//! rest is converted to Ethernet for the stack.

mod check;
mod deliver;
mod open;
mod outcome;

use deliver::Checked;
pub use outcome::{Rx, RxDrop};
