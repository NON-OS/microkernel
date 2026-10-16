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

//! What a join reports back, and the status code each way it can end maps to.
//! Codes -1 to -6 keep the meanings the client already shows; the rest name
//! the WPA3 and security outcomes a person can act on (the network now offers
//! only WPA2 though it was saved as WPA3, the passphrase is not one, the SAE
//! confirm failed, message 3 did not match the beacon).

mod codes;
mod map;
mod report;

pub use codes::*;
pub use map::failure_code;
pub use report::ConnectResult;
