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

//! A request from the shared crate's client half, answered by the capsule,
//! and its reply read back as Settings reads it.

use super::fixture::{request, TestBoard, ID};
use crate::nbtc::answer;
use crate::nbtc::who::Caller::Settings;
use crate::nbtc::wire::REPLY_MAX;
use nonos_bt_proto as p;

/// A request from the client encoder, answered, and its reply read back.
pub fn call(b: &mut TestBoard, op: u16, body: &[u8], now: u64) -> (i32, Vec<u8>) {
    let mut frame = [0u8; 64];
    let n = p::request(&mut frame, op, ID, body).unwrap();
    assert_eq!(frame[..n], request(op, body), "both encoders lay the same frame");
    let mut tx = [0u8; REPLY_MAX];
    let got = answer(b, Settings, &frame[..n], now, &mut tx);
    assert_eq!(p::read_reply(&tx[..got], op, ID ^ 1), None, "another request's reply");
    assert_eq!(p::read_reply(&tx[..got], op ^ 1, ID), None, "another op's reply");
    let (status, data) = p::read_reply(&tx[..got], op, ID).unwrap();
    (status, data.to_vec())
}
