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

//! Choose how to join a network from what its RSN element offers and what the
//! person allowed. SAE is taken whenever the access point offers it (a
//! transition-mode network offers SAE and PSK; joining it with PSK would hand
//! out a handshake an eavesdropper can run a dictionary against), PSK-SHA256 or
//! PSK otherwise. A network saved as WPA3 is joined with SAE or not at all, so
//! an access point that answers to the same name with PSK only is refused as a
//! downgrade instead of being trusted. Only CCMP-128 is run: a TKIP group
//! cipher would leave broadcast and multicast frames (ARP, DHCP) undecryptable,
//! so such a network is refused with its own reason.

mod choose;
mod kinds;
mod policy;

pub use choose::select;
pub use kinds::{Pmf, SelectError, Selection};
pub use policy::JoinPolicy;
