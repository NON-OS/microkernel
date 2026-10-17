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

//! The purchase path's decisions, split from its screens and calls: the
//! reading's order, Buy's, Open's and Fund's gates, the fresh address, the
//! meter's order id and deposit recomputed, and the consent's terms. They
//! name their siblings through `super`, which these uses stand in for.

use super::{buy, fees_kind, held, read, voucher};

#[path = "../../../../capsule_wallet_nonos/src/wallet/order/buy_gate.rs"]
pub mod buy_gate;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/deposit_check.rs"]
pub mod deposit_check;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/fresh_reply.rs"]
pub mod fresh_reply;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/fund_ask.rs"]
pub mod fund_ask;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/open_gate.rs"]
pub mod open_gate;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/pending_step.rs"]
pub mod pending_step;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/reading_step.rs"]
pub mod reading_step;
#[path = "../../../../capsule_wallet_nonos/src/wallet/order/terms/mod.rs"]
pub mod terms;
