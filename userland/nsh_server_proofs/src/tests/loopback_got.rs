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

//! What the loopback client saw: its refusal, its events and every byte each
//! way.

use nsh_client::{Event as C, Refusal};

pub type Loop = nsh_client::Client<nsh_client::NonosAttester, crate::kit::user::User>;

pub struct Got {
    pub err: Option<Refusal>,
    pub events: Vec<C>,
    pub c2s: Vec<u8>,
    pub s2c: Vec<u8>,
    pub th: Option<[u8; 64]>,
}
