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

//! A policy is vouched for and verified under its own domain, and nothing
//! else this machine signs, proves or has a publisher prove can begin with
//! one: so the measurement a front takes as a vouched policy is never a
//! program's.

const VOUCH: &str = include_str!("../../../capsule_linux/src/linux/install/record/vouch.rs");
const MARKET_BIND: &str = include_str!("../../../capsule_linux/src/linux/install/market/bind.rs");
const BIND_ARGS: &str =
    include_str!("../../../capsule_linux/src/linux/install/market/bind_args.rs");
const APP_BIND: &str = include_str!("../../../capsule_linux/src/linux/app/bind.rs");
const LOCAL: &str = include_str!("../../../capsule_linux/src/linux/attest_local.rs");
const PUBLISHER: &str = include_str!("../../../capsule_linux/src/linux/attest_publisher.rs");

fn flat(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<String>()
        .split_whitespace()
        .collect()
}

fn before(src: &str, first: &str, then: &str) -> bool {
    let code = flat(src);
    match (code.find(first), code.find(then)) {
        (Some(a), Some(b)) => a < b,
        _ => false,
    }
}

#[test]
fn nothing_minted_plain_begins_with_a_domain() {
    assert!(before(VOUCH, "ifin_a_domain(bytes){returnNone;}sign(bytes)", "fnsign("));
    let code = flat(VOUCH);
    let calls = code
        .match_indices("sign(")
        .filter(|(at, _)| !code[..*at].ends_with(|c: char| c.is_alphanumeric() || c == '_'))
        .count();
    assert_eq!(calls, 2, "mint and sign_beside_in are the only signers");
    assert!(code.contains("in_a_domain(domain).then(||sign(&[domain,bytes].concat()))"));
}

#[test]
fn a_policy_is_signed_and_verified_under_its_domain() {
    assert!(flat(MARKET_BIND)
        .contains("sign_beside_in(VOUCH_DOMAIN,&path,&policy.encode(),POLICY_TRAILER)"));
    assert!(flat(MARKET_BIND).contains("usenonos_linux_policy::{vouch::VOUCH_DOMAIN,"));
    assert!(flat(BIND_ARGS).contains("sign_beside_in(VOUCH_DOMAIN,path,&[policy,args].concat()"));
    let app = flat(APP_BIND);
    assert!(app.contains("letvouched=[VOUCH_DOMAIN,&policy[..]].concat();"));
    assert!(app.contains("mk_local_verify(&vouched,GUEST_CAPS,&trailer)"));
    assert!(!app.contains("mk_local_verify(&policy,"));
}

#[test]
fn no_program_beginning_with_a_domain_is_proved() {
    assert!(before(LOCAL, "ifin_a_domain(image){", "mk_local_verify(image,"));
    assert!(before(PUBLISHER, "ifin_a_domain(image){", "mk_capsule_verify(&req,"));
}
