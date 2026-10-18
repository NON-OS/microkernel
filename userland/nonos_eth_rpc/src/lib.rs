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

//! Ethereum JSON-RPC as bytes, no_std: requests, answers, eth_getLogs by
//! address and topic0, HTTP/1.1. From the wallet's rpc module, plus logs.

#![no_std]
extern crate alloc;

mod append;
mod batch_id;
mod broadcast_error;
mod fees;
mod find_result;
mod hex_digit;
mod hex_text;
mod http_answer;
mod http_body;
mod http_parts;
mod http_post;
mod json_scan;
mod object_id;
mod parse_call_word;
mod parse_fee_history;
mod parse_hash32;
mod parse_logs;
mod parse_quantity32;
mod parse_receipt_logs;
mod parse_receipt_ok;
mod parse_u64;
mod request_balance;
mod request_batch;
mod request_broadcast;
mod request_chain_id;
mod request_estimate_gas;
mod request_eth_call;
mod request_fee;
mod request_logs;
mod request_nonce;
mod request_receipt;

pub use append::append_quantity;
pub use batch_id::object_for_id;
pub use broadcast_error::{broadcast_error, NodeSaid, UNNAMED};
pub use fees::{fees, MIN_TIP_WEI};
pub use hex_text::{hex_bytes, quantity};
pub use http_answer::{http_answer, Http};
pub use json_scan::{objects, string_field};
pub use parse_logs::{parse_logs, Log};
pub use parse_quantity32::parse_quantity32;
pub use parse_receipt_ok::{parse_receipt_block, parse_receipt_for, parse_receipt_ok};
pub use parse_u64::parse_u64;
pub use request_fee::{request_fee, request_fee_history};
pub use request_logs::{block_pages, request_logs, LOG_PAGE_BLOCKS};
pub use request_nonce::{request_nonce, request_nonce_latest};
pub use request_receipt::{request_block_number, request_receipt};
pub use {http_body::http_body, http_post::http_post};
pub use {parse_call_word::parse_call_word, parse_hash32::parse_hash32};
pub use {parse_fee_history::parse_fee_history, parse_receipt_logs::parse_receipt_logs};
pub use {request_balance::request_balance, request_batch::request_batch};
pub use {request_broadcast::request_broadcast, request_chain_id::request_chain_id};
pub use {request_estimate_gas::request_estimate_gas, request_eth_call::request_eth_call};
