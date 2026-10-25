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

//! Every parser on the gen3 path, against the bundled firmware and against
//! malformed input: the firmware description, firmware selection, received
//! packets, ALIVE, the NVM reply, received management frames. The bundled
//! images are iwlwifi-so-a0-gf-a0-86.ucode (sha256 cea0aaeb034aadb36cde1d7b6f2a
//! 0f38cb78209cbe81a40d973318f820ebdd0c) and iwlwifi-so-a0-hr-b0-84.ucode
//! (sha256 20ce23a3c723efddbd909a239b99f9c9451c1f4fbcbdb1568eca9057853aa110);
//! the values pinned for them were read with an independent Python TLV dump.

mod ax210;
mod frames;
mod images;
mod malformed;
mod nvm;
mod packets;
mod select;
mod so;
