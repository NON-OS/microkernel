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

//! The MLME against a simulated access point: a WPA2 join and a WPA3-SAE join
//! (hash-to-element and hunting and pecking) run to Connected with the keys
//! the AP derives; the client picks SAE on a transition network; a network
//! saved as WPA3 is refused when it shows PSK only; frames from another BSS
//! or for another station are ignored; refusals, a deauthentication, an
//! open network, a TKIP group cipher and an HT-only AP each end the join with
//! their own reason; and the association request offers the AP's own rates.

mod assoc;
mod base;
mod frames;
mod ignored;
mod sae;
mod sae_forged;
mod unjoinable;
mod wpa2;
mod wpa3;
