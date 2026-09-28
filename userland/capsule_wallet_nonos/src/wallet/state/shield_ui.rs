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

/*
 * What the Shield screens remember between paints: which one is up and what
 * the holder picked on it. The history and the running proof come from the
 * shield service, so a screen never shows a state no service reported.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::shield_log::{Entry, Job};

pub const SHIELD_HOME: u8 = 0;
pub const SHIELD_DEPOSIT: u8 = 1;
pub const SHIELD_SEND: u8 = 2;
pub const SHIELD_WITHDRAW: u8 = 3;
pub const SHIELD_REVIEW: u8 = 4;
pub const SHIELD_PROVING: u8 = 5;
pub const SHIELD_HISTORY: u8 = 6;
pub const SHIELD_NETWORK: u8 = 7;

/* Assets by index: 0 is ETH. */
pub const ASSET_NOX: u8 = 1;

pub const FIELD_TO: u8 = 0;
pub const FIELD_AMOUNT: u8 = 1;
pub const FIELD_OVERRIDE: u8 = 2;

#[derive(Default)]
pub struct ShieldUi {
    pub screen: u8,
    /* The screen the review was opened from, so back returns to it. */
    pub from: u8,
    pub asset: u8,
    pub size: Option<u8>,
    pub to: String,
    pub amount: String,
    pub override_text: String,
    pub focus: u8,
    /* The private receive address, once the shield service has derived it. */
    pub nox1: Option<String>,
    /* Pool growth since the newest note, as the scanner last counted it. */
    pub leaves_since: Option<u32>,
    pub hours_since: Option<u32>,
    pub history: Vec<Entry>,
    pub job: Option<Job>,
    pub failure: Option<&'static str>,
    /* The absence banner, dismissed until Shield is next opened. */
    pub absent_hidden: bool,
}
