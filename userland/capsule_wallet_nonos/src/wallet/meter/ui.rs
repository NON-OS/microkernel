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
 * What the wallet holds of the meter (B8): its last ACCOUNT reply, read when
 * the meter screen opens and on Refresh, never in the background.
 */

use alloc::string::String;

use nonos_payment_proto::Account;

#[derive(Default)]
pub struct MeterUi {
    /* The meter's first ACCOUNT page, or None when it did not answer. */
    pub account: Option<Account>,
    /* Whether the meter was asked since the screen opened. */
    pub asked: bool,
    /* The budget form, as typed, and the field that takes keys. */
    pub cap_text: String,
    pub days_text: String,
    pub focus: u8,
    /* The channel row picked, the top-up as typed, the job running, and
     * the last job's line once it was sent. */
    pub picked: Option<u8>,
    pub topup_text: String,
    pub job: Option<super::job::MeterJob>,
    pub news: Option<String>,
    /* A purchase the store named (B8), and when to ask the mailbox next. */
    pub purchase: Option<alloc::boxed::Box<crate::wallet::order::intake::Pending>>,
    pub take_at: u64,
    /* The one ASYNC request to settle (v1.4.53), for a dispute or refund. */
    pub settle: crate::wallet::settle::SettleCall,
    /* Refunds and reclaims landed at fresh addresses (v1.4.52). */
    pub landed: crate::wallet::shield::landed::Landed,
}
