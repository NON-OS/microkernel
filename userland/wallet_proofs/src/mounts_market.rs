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

//! The send, market, meter and Shield files, named at the crate root.

#[path = "../../capsule_wallet_nonos/src/wallet/attest/mod.rs"]
pub mod attest;
#[path = "../../capsule_wallet_nonos/src/wallet/clock/unix.rs"]
pub mod clock_unix;
#[path = "../../capsule_wallet_nonos/src/wallet/send/exact.rs"]
pub mod exact;
#[path = "../../capsule_wallet_nonos/src/wallet/send/fees.rs"]
pub mod fees;
#[path = "../../capsule_wallet_nonos/src/wallet/shield/fund_meter.rs"]
pub mod fund_meter;
#[path = "../../capsule_wallet_nonos/src/wallet/hex_id.rs"]
pub mod hex_id;
/// The intent files, with the names the tests use.
#[path = "intent/mod.rs"]
pub mod intent;
#[path = "../../capsule_wallet_nonos/src/wallet/shield/kept_names.rs"]
pub mod kept_names;
#[path = "../../capsule_wallet_nonos/src/wallet/meter/budget_form.rs"]
pub mod meter_budget;
#[path = "../../capsule_wallet_nonos/src/wallet/meter/channel.rs"]
pub mod meter_channel;
#[path = "../../capsule_wallet_nonos/src/wallet/meter/job.rs"]
pub mod meter_job;
#[path = "../../capsule_wallet_nonos/src/wallet/meter/rows.rs"]
pub mod meter_rows;
#[path = "../../capsule_wallet_nonos/src/wallet/meter/topup.rs"]
pub mod meter_topup;
/// The order's pure parts (order/mod.rs).
#[path = "order/mod.rs"]
pub mod order;
#[path = "../../capsule_wallet_nonos/src/wallet/publisher/mod.rs"]
pub mod publisher;
#[path = "../../capsule_wallet_nonos/src/wallet/send/sent.rs"]
pub mod sent;
#[path = "../../capsule_wallet_nonos/src/wallet/settle/heard.rs"]
pub mod settle_heard;
#[path = "shield/mod.rs"]
pub mod shield;
#[path = "../../capsule_wallet_nonos/src/wallet/screen/shield/typed.rs"]
pub mod shield_typed;
#[path = "../../capsule_wallet_nonos/src/wallet/typed/mod.rs"]
pub mod typed;
