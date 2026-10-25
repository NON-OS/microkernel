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
 * What the Shield screens remember between paints: which one is up, what
 * the holder picked on it, and what the shield service last reported. A
 * screen never shows a balance, a fee or a state no service answered.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::shield_log::Entry;
pub use super::shield_ui_consts::*;
pub use super::shield_ui_spend::Spend;
pub use super::shield_ui_types::{Held, Quote, Review, Waiting};

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
    /* The service holds this account's shield store open. */
    pub opened: bool,
    /* The private receive address, once the service has derived it. */
    pub nox1: Option<String>,
    /* ETH then NOX. */
    pub held: [Option<Held>; 2],
    /* Notes to land and minutes to pass before every note may be spent. */
    pub wait_left: Option<(u32, u32)>,
    pub waiting: Option<Waiting>,
    /* The running job's time and the prover's phase (`shield_ui_parts`). */
    pub job: super::shield_ui_parts::JobProgress,
    pub review: Option<Review>,
    pub quote: Option<Quote>,
    /* Who a withdrawal pays (`shield_ui_parts`). */
    pub payee: super::shield_ui_parts::Payee,
    /* The coin and amount of the spend being proved, as confirmed. */
    pub proving: Option<(u8, String)>,
    pub spend: Option<Spend>,
    /* Spends proved before the last and not landed yet, oldest first. */
    pub earlier: Vec<Spend>,
    /* The spend a settlement under review lands: None for the last one. */
    pub settling_id: Option<String>,
    pub history: Vec<Entry>,
    /* Older entries the service's last state reply had no room for. */
    pub history_cut: u32,
    pub failure: Option<String>,
    /* A line of news: a deposit sent, a payment landed. */
    pub news: Option<String>,
    /* The absence banner, dismissed until Shield is next opened. */
    pub absent_hidden: bool,
    /* How the service has been answering (`shield_ui_parts`). */
    pub link: super::shield_ui_parts::ServiceLink,
}
