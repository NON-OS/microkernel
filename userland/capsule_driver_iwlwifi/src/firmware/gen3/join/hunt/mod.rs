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

//! Finding the network before a join. The join needs the network's own
//! beacon (the MLME reads the BSSID, the RSN element and the rates out of it)
//! and the channel it was heard on, so a passive sweep runs over the NVM's
//! channels until a beacon or probe response names the network, and is then
//! stopped. Nothing is transmitted: a network that hides its name is not
//! found. A background sweep already in flight is stopped first so the hunt
//! hears every channel. Every frame heard is handed on for the scan list
//! too. The hunt is bounded by the sweep's own budget, and it never returns
//! with a sweep still in flight: the join needs the radio to itself.

mod find;
mod kinds;
mod stop;

pub use find::hunt;
pub use kinds::HuntEnd;
