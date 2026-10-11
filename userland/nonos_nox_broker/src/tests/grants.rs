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

use nonos_nox_license::ToolId;

use super::fixtures::{paying, price, terms, to, BUYER};
use crate::issue::{issue, Terms};
use crate::spent::SpentSet;

const RECON: u32 = ToolId::Recon.id();

#[test]
fn a_paid_receipt_issues_a_single_use_grant() {
    let mut spent = SpentSet::new();
    let tx = [0xAB; 32];
    let r = paying(price());
    let out = issue(
        &mut spent,
        RECON,
        &to(),
        tx,
        r.as_bytes(),
        &Terms { now: 1_000, ttl_secs: 3_600, nonce: [7; 8], device: [0; 32] },
    )
    .expect("valid payment issues");
    assert_eq!(out.entitlement.buyer, BUYER);
    assert_eq!(out.entitlement.uses, 1);
    assert_eq!(out.entitlement.tool_id, RECON);
    assert_eq!(out.entitlement.tx_hash, tx);
    assert_eq!(out.entitlement.expiry, 1_000 + 3_600);
    assert!(spent.contains(&tx));
}

#[test]
fn paying_a_multiple_buys_a_bundle() {
    let mut spent = SpentSet::new();
    let r = paying(price() * 4);
    let out = issue(&mut spent, RECON, &to(), [1; 32], r.as_bytes(), &terms()).expect("valid");
    assert_eq!(out.entitlement.uses, 4);
    // ttl 0 means no expiry.
    assert_eq!(out.entitlement.expiry, 0);
}

#[test]
fn a_partial_overpayment_rounds_down_to_whole_uses() {
    let mut spent = SpentSet::new();
    // 2.5x the price: two whole uses, remainder is a tip.
    let r = paying(price() * 5 / 2);
    let out = issue(&mut spent, RECON, &to(), [2; 32], r.as_bytes(), &terms()).expect("valid");
    assert_eq!(out.entitlement.uses, 2);
}

#[test]
fn device_binding_is_carried_into_the_grant() {
    let mut spent = SpentSet::new();
    let device = [0x5A; 32];
    let r = paying(price());
    let out = issue(&mut spent, RECON, &to(), [6; 32], r.as_bytes(), &Terms { device, ..terms() })
        .expect("valid");
    assert_eq!(out.entitlement.device, device);
}
