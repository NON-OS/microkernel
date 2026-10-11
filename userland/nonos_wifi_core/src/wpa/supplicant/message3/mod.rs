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

//! Message 3 of the four-way handshake (IEEE Std 802.11-2020, 12.7.6.4), the
//! first time and repeated. The first verifies, checks the RSNE (and RSNXE)
//! against the beacon, takes the group keys, and answers with message 4. A
//! repeat after that (the AP did not get message 4) is answered with message 4
//! again and changes nothing else: reinstalling the pairwise key on a repeat is
//! the key-reinstallation attack, so the keys stay as they are.

mod again;
mod check;
mod first;
