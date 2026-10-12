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

//! f4, f5, f6 and g2 against the sample data of Vol 3, Part H, Appendix D,
//! as recalled: no copy of the specification is reachable here, so these
//! byte strings are from memory of that appendix and say so. They share
//! values across functions (f4's U is the debug public key's X, f5's N1 is
//! f4's X, f6 takes f5's MacKey), which a misremembered octet would break.

use crate::hex::arr;
use nonos_bt_smp::funcs::{addr56, f4, f5, f6, g2, passkey_display};
use nonos_bt_wire::addr::{AddrType, BdAddr};

const U: &str = "20b003d2 f297be2c 5e2c83a7 e9f9a5b9 eff49111 acf4fddb cc030148 0e359de6";
const V: &str = "55188b3d 32f6bb9a 900afcfb eed4e72a 59cb9ac2 f19d7cfb 6b4fdd49 f47fc5fd";
const N1: &str = "d5cb8454 d177733e ffffb2ec 712baeab";
const N2: &str = "a6e8e7cc 25a75f6e 216583f7 ff3dc4cf";
const A1: &str = "00561237 37bfce";
const A2: &str = "00a71370 2dcfc1";
const MAC_KEY: &str = "2965f176 a1084a02 fd3f6a20 ce636e20";

#[test]
fn f4_sample() {
    let out = f4(&arr(U), &arr(V), &arr(N1), 0);
    assert_eq!(out, arr("f2c916f1 07a9bd1c f1eda1be a974872d"));
}

#[test]
fn f5_sample() {
    let w = arr("ec0234a3 57c8ad05 341010a6 0a397d9b 99796b13 b4f866f1 868d34f3 73bfa698");
    let (mac_key, ltk) = f5(&w, &arr(N1), &arr(N2), &arr(A1), &arr(A2));
    assert_eq!(mac_key, arr(MAC_KEY));
    assert_eq!(ltk, arr("69867911 69d7cd23 980522b5 94750a38"));
}

#[test]
fn f6_sample() {
    let r = arr("12a3343b b453bb54 08da42d2 0c2d0fc8");
    let out = f6(&arr(MAC_KEY), &arr(N1), &arr(N2), &r, &arr("010102"), &arr(A1), &arr(A2));
    assert_eq!(out, arr("e3c47398 9cd0e8c5 d26c0b09 da958f61"));
}

#[test]
fn g2_sample() {
    let out = g2(&arr(U), &arr(V), &arr(N1), &arr(N2));
    assert_eq!(out, 0x2f9ed5ba);
    assert_eq!(passkey_display(out), 0x2f9ed5ba % 1_000_000);
}

#[test]
fn addresses_take_the_f5_form() {
    let a1 = BdAddr([0xce, 0xbf, 0x37, 0x37, 0x12, 0x56]);
    assert_eq!(addr56(AddrType::Public, &a1), arr::<7>(A1));
    assert_eq!(addr56(AddrType::Random, &a1)[0], 0x01);
}
