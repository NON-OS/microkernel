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

//! A spend followed from its proof to its landing.

use alloc::string::String;
use alloc::vec::Vec;

use super::shield_log::Kind;

/* A spend from its proof to its landing: the one proved last, or one kept
 * before it, which the service names by `id`. */
#[derive(Clone)]
pub struct Spend {
    /* The number the service keeps an earlier spend under; None for the
     * last one, and for an earlier one until the service has named it. */
    pub id: Option<String>,
    pub kind: Kind,
    pub asset: u8,
    pub amount: String,
    pub published: bool,
    pub state: String,
    pub minutes: u32,
    pub tx: Option<String>,
    pub self_settle: bool,
    pub refusal: Option<String>,
    pub weakened: Vec<String>,
}
