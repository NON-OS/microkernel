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
//! The wallet's JSON-RPC is nonos_eth_rpc's; only its start-up self check is its own.

mod self_check;

pub use nonos_eth_rpc::{broadcast_error, NodeSaid};
pub use nonos_eth_rpc::{http_answer, http_post, Http};
pub use nonos_eth_rpc::{object_for_id, request_batch};
pub use nonos_eth_rpc::{parse_call_word, parse_hash32, parse_quantity32, parse_u64};
pub use nonos_eth_rpc::{parse_receipt_block, parse_receipt_for};
pub use nonos_eth_rpc::{request_balance, request_broadcast, request_chain_id};
pub use nonos_eth_rpc::{request_block_number, request_receipt};
pub use nonos_eth_rpc::{request_estimate_gas, request_eth_call};
pub use nonos_eth_rpc::{request_fee, request_fee_history};
pub use nonos_eth_rpc::{request_nonce, request_nonce_latest};
pub use self_check::self_check;
