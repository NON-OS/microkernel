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
//! A receipt written: the part the challenge covers, then the doc that

//! The lines a verified receipt prints for what else the machine ran: how
//! many capsules, and each that held Network, which a box's own run never
//! needs to share with anyone.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::cli::hex;
use crate::verify::Ran;

pub fn lines(ran: &[Ran]) -> Vec<String> {
    let mut out = Vec::new();
    out.push(format!("ran      {} capsules, every one the signed registry names", ran.len()));
    let net = ran.iter().filter(|r| r.network());
    let held: Vec<String> = net
        .map(|r| {
            format!("network  pid {} {} authority {}", r.pid, hex(&r.measurement), r.authority)
        })
        .collect();
    match held.is_empty() {
        true => out.push("network  no capsule held Network".into()),
        false => out.extend(held),
    }
    out
}
