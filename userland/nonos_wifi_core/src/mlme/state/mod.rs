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

//! MLME state: the target network, the person's policy, the randomness the
//! join consumes, the BSS and security chosen from its beacon, and the SAE
//! exchange and handshake that follow.

mod fields;
mod keys;
mod kinds;
mod own;
mod read;
mod request;
mod start;

pub use fields::Mlme;
pub use kinds::{MlmeOutput, MlmeState};
pub use request::{Entropy, JoinRequest, SAE_ENTROPY, SECRET_MAX};
