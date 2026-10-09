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

use super::shield_log::{Entry, Kind};

pub const SHIELD_HOME: u8 = 0;
pub const SHIELD_DEPOSIT: u8 = 1;
pub const SHIELD_SEND: u8 = 2;
pub const SHIELD_WITHDRAW: u8 = 3;
pub const SHIELD_REVIEW: u8 = 4;
pub const SHIELD_PROVING: u8 = 5;
pub const SHIELD_HISTORY: u8 = 6;
pub const SHIELD_NETWORK: u8 = 7;
/* Never shown on its own: the review's origin when the holder lands a
 * payment from their own account because no lander took it. */
pub const SHIELD_SETTLE: u8 = 8;

/* Assets by index: 0 is ETH. */
pub const ASSET_NOX: u8 = 1;

pub const FIELD_TO: u8 = 0;
pub const FIELD_AMOUNT: u8 = 1;
pub const FIELD_OVERRIDE: u8 = 2;

/* One coin's shielded holding, as the service wrote it. */
#[derive(Clone, Default)]
pub struct Held {
    pub spendable: String,
    pub pending: String,
}

/* The one job the service runs for this wallet, and when it was asked. */
#[derive(Clone, Copy)]
pub struct Waiting {
    pub op: u16,
    pub since_ms: i64,
}

/* A deposit, an approval or a self-settlement, checked and held by the
 * service under `id` until confirmed. */
#[derive(Clone, Default)]
pub struct Review {
    pub id: String,
    pub approval: bool,
    pub amount: String,
    pub pool_fee: String,
    pub shielded: String,
    pub max_network_fee: String,
    pub valid_for: String,
}

/* What a private payment or a withdrawal costs, read from the pool. */
#[derive(Clone, Default)]
pub struct Quote {
    pub network_fee: String,
    pub protocol_fee: String,
    pub total_fee: String,
}

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
    /* Seconds the running job has taken, as of the last tick. */
    pub elapsed_s: u32,
    /* The prover's own phase and how far through it, in thousandths, when
     * the service reports them. */
    pub phase: Option<String>,
    pub permille: Option<u16>,
    pub review: Option<Review>,
    pub quote: Option<Quote>,
    /* The fresh public address a withdrawal pays. */
    pub withdraw_to: Option<String>,
    /* The coin and amount of the spend being proved, as confirmed. */
    pub proving: Option<(u8, String)>,
    pub spend: Option<Spend>,
    /* Spends proved before the last and not landed yet, oldest first. */
    pub earlier: Vec<Spend>,
    /* The spend a settlement under review lands: None for the last one. */
    pub settling_id: Option<String>,
    pub last_follow_ms: i64,
    pub last_sync_ms: i64,
    pub history: Vec<Entry>,
    /* Older entries the service's last state reply had no room for. */
    pub history_cut: u32,
    pub failure: Option<String>,
    /* A line of news: a deposit sent, a payment landed. */
    pub news: Option<String>,
    /* The absence banner, dismissed until Shield is next opened. */
    pub absent_hidden: bool,
    /* The service did not answer in time, as one still starting on a slow
     * machine may not: opening is asked again by itself at this uptime, with
     * a longer wait each time, and `tries` says how many times so far. */
    pub retry_at_ms: Option<i64>,
    pub tries: u8,
    /* The service keeps this store in memory, for this boot only. */
    pub live: bool,
    /* Result asks in a row the service did not answer, and the uptime of
     * the next one: a service that stops answering is asked less often,
     * and after `job::MISSES` the job is said lost. */
    pub misses: u8,
    pub poll_at_ms: i64,
}
