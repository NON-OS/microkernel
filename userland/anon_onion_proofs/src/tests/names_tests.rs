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


//! The names layer's own rules: what list is used, what is refused, and how
//! a name is held to its first service.

use std::string::String;
use std::vec::Vec;

use super::names_doc::{address_of, header, sign, sign_as, signer};
use crate::onion::address::{encode, parse};
use crate::onion::names::document::{is_short_name, NameError};
use crate::onion::names::{defaults, may_replace, normal, verify, NameList, Pins, Resolve, LIST_MAX, NAMES_MAX};
use crate::onion::names::pins::PINS_MAX;

/// 2026-10-03 12:00:00 UTC.
const NOW: u64 = 1_791_028_800;
const PUBLISHED: &str = "2026-10-01 00:00:00";
const VALID: &str = "2026-11-01 00:00:00";

fn target(label: &str) -> String {
    address_of(&signer(label).verifying_key().to_bytes())
}

fn trusted() -> Vec<[u8; 32]> {
    std::vec![signer("dns").verifying_key().to_bytes()]
}

fn good() -> Vec<u8> {
    let body = header(PUBLISHED, VALID)
        + &std::format!("lander.anyone {}\n", target("lander"))
        + &std::format!("Wallet.Shop.anyone\t{}\n", target("shop"));
    sign(&body, &signer("dns"))
}

#[test]
fn a_signed_current_list_verifies_and_names_are_lowercased() {
    let list = verify(&good(), &trusted(), NOW).expect("verifies");
    assert_eq!(list.entries.len(), 2);
    assert_eq!(list.lookup(b"lander.anyone"), parse(target("lander").as_bytes()));
    assert_eq!(list.lookup(b"wallet.shop.anyone"), parse(target("shop").as_bytes()));
    assert_eq!(list.lookup(b"Wallet.Shop.anyone"), None, "lookups take the normal form");
    assert_eq!(list.valid_until - list.published, 31 * 86400);
}

#[test]
fn the_forks_own_layout_verifies() {
    // test_anyone_hosts_update.c's shape: digest line, tab separator.
    let body = header(PUBLISHED, VALID)
        + &std::format!("one.anyone.anyone {}\n", target("one"))
        + &std::format!("two.anyone.anyone\t{}\n", target("two"))
        + "anyone-hosts-digest sha256 abc\n";
    assert_eq!(verify(&sign(&body, &signer("dns")), &trusted(), NOW).unwrap().entries.len(), 2);
}

#[test]
fn only_the_six_services_may_sign_in_production() {
    assert_eq!(verify(&good(), &defaults::signers(), NOW).err(), Some(NameError::UntrustedSigner));
    let other = sign(&(header(PUBLISHED, VALID) + &std::format!("x.anyone {}\n", target("x"))), &signer("mallory"));
    assert_eq!(verify(&other, &trusted(), NOW).err(), Some(NameError::UntrustedSigner));
}

#[test]
fn naming_a_trusted_signer_without_its_key_fails_the_signature() {
    let dns = address_of(&signer("dns").verifying_key().to_bytes());
    let forged = sign_as(&(header(PUBLISHED, VALID) + &std::format!("x.anyone {}\n", target("x"))), &signer("mallory"), &dns);
    assert_eq!(verify(&forged, &trusted(), NOW).err(), Some(NameError::BadSignature));
}

#[test]
fn every_single_byte_change_is_refused() {
    let doc = good();
    for at in 0..doc.len() {
        for flip in [0x01u8, 0x20] {
            let mut d = doc.clone();
            d[at] ^= flip;
            if d[at] == b'\n' && doc[at] != b'\n' {
                continue;
            }
            assert!(verify(&d, &trusted(), NOW).is_err(), "flip {flip:#x} at {at}: {:?}", String::from_utf8_lossy(&d[at.saturating_sub(10)..(at + 10).min(d.len())]));
        }
    }
    for n in 0..doc.len() - 1 {
        assert!(verify(&doc[..n], &trusted(), NOW).is_err(), "cut to {n}");
    }
}

#[test]
fn time_is_held() {
    assert_eq!(verify(&good(), &trusted(), NOW + 40 * 86400).err(), Some(NameError::Expired));
    let backwards = sign(&(header(VALID, PUBLISHED) + &std::format!("x.anyone {}\n", target("x"))), &signer("dns"));
    assert_eq!(verify(&backwards, &trusted(), NOW).err(), Some(NameError::Expired), "published after valid-until");
    let undated = String::from("anyone-hosts-version 1\npublished 2026-10-01 00:00:00\n") + &std::format!("x.anyone {}\n", target("x"));
    assert_eq!(verify(&sign(&undated, &signer("dns")), &trusted(), NOW).err(), Some(NameError::Unsigned), "no valid-until");
    let unpublished = String::from("anyone-hosts-version 1\nvalid-until 2026-11-01 00:00:00\n") + &std::format!("x.anyone {}\n", target("x"));
    assert_eq!(verify(&sign(&unpublished, &signer("dns")), &trusted(), NOW).err(), Some(NameError::Unsigned), "no published");
}

#[test]
fn shape_and_content_are_held() {
    let x = target("x");
    let case = |body: String| verify(&sign(&body, &signer("dns")), &trusted(), NOW).err();
    assert_eq!(case(header(PUBLISHED, VALID)), Some(NameError::Empty));
    assert_eq!(case(header(PUBLISHED, VALID).replace("version 1", "version 2") + &std::format!("x.anyone {x}\n")), Some(NameError::Version));
    assert_eq!(case(header(PUBLISHED, VALID) + "surprise keyword\n"), Some(NameError::Malformed));
    assert_eq!(case(header(PUBLISHED, VALID) + "published 2026-10-02 00:00:00\n" + &std::format!("x.anyone {x}\n")), Some(NameError::Malformed), "a keyword twice");
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("x.anyone {x}\nx.anyone {}\n", target("y"))), Some(NameError::Ambiguous));
    assert!(case(header(PUBLISHED, VALID) + &std::format!("x.anyone {x}\nx.anyone {x}\n")).is_none(), "the same line twice is one name");
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("{x} {x}\n")), Some(NameError::Malformed), "an address is not a name");
    let mut bad = x.clone().into_bytes();
    bad[3] = if bad[3] == b'a' { b'b' } else { b'a' };
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("x.anyone {}\n", String::from_utf8(bad).unwrap())), Some(NameError::Malformed), "a checksum that fails");
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("x.anyone {x} extra\n")), Some(NameError::Malformed));
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("x.anyone sub.{x}\n")), Some(NameError::Malformed), "a target with a label in front");
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("x_y.anyone {x}\n")), Some(NameError::Malformed));
    assert_eq!(case(header(PUBLISHED, VALID) + &std::format!("x.anyone\r\n{x}\n")), Some(NameError::Malformed), "CR is refused");
    let unsigned = header(PUBLISHED, VALID) + &std::format!("x.anyone {x}\n");
    assert_eq!(verify(unsigned.as_bytes(), &trusted(), NOW).err(), Some(NameError::Unsigned));
    assert_eq!(verify(b"lander.anyone abc.anyone\n", &trusted(), NOW).err(), Some(NameError::Unsigned), "the fork's legacy unsigned file");
}

#[test]
fn size_is_held() {
    let x = target("x");
    let mut many = header(PUBLISHED, VALID);
    for i in 0..=NAMES_MAX {
        many += &std::format!("n{i}.anyone {x}\n");
    }
    let doc = sign(&many, &signer("dns"));
    assert!(doc.len() > LIST_MAX);
    assert_eq!(verify(&doc, &trusted(), NOW).err(), Some(NameError::TooLarge));
    let mut fits = header(PUBLISHED, VALID);
    for i in 0..500 {
        fits += &std::format!("n{i}.anyone {x}\n");
    }
    assert_eq!(verify(&sign(&fits, &signer("dns")), &trusted(), NOW).unwrap().entries.len(), 500);
}

#[test]
fn the_six_services_check_out_and_name_themselves() {
    assert_eq!(defaults::signers().len(), 6, "every hardcoded address carries a valid checksum");
    let builtin = defaults::list();
    assert_eq!(builtin.entries.len(), 6);
    for (name, address) in defaults::MAPPING {
        assert_eq!(builtin.lookup(name.as_bytes()), parse(address.as_bytes()));
        assert_eq!(&encode(&parse(address.as_bytes()).unwrap())[..], address.as_bytes(), "encode is parse's inverse");
    }
    let lander = b"iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone";
    assert_eq!(&encode(&parse(lander).unwrap())[..], &lander[..]);
}

#[test]
fn what_a_short_name_is() {
    for good in ["lander.anyone", "a.b-c.anyone", "dns-live-1.anyone.anyone", "x1.anyone"] {
        assert!(is_short_name(good.as_bytes()), "{good}");
    }
    let address = target("x");
    for bad in [".anyone", "anyone", "x..anyone", "-.anyone ", "X.anyone", "x_y.anyone", address.as_str(), "x.onion"] {
        assert!(!is_short_name(bad.as_bytes()), "{bad}");
    }
    assert_eq!(normal(b"Lander.ANYONE.."), b"lander.anyone");
}

fn list(published: u64, entries: &[(&str, &str)]) -> NameList {
    NameList {
        published,
        valid_until: NOW + 86400,
        entries: entries.iter().map(|(n, t)| (n.as_bytes().to_vec(), parse(target(t).as_bytes()).unwrap())).collect(),
    }
}

#[test]
fn a_name_keeps_its_first_service_for_the_boot() {
    let builtin = defaults::list();
    let mut pins = Pins::default();
    let first = list(1, &[("lander.anyone", "a")]);
    let moved = list(2, &[("lander.anyone", "b")]);
    let a = pins.resolve(b"lander.anyone", &builtin, Some(&first), NOW).unwrap();
    assert_eq!(pins.resolve(b"lander.anyone", &builtin, Some(&first), NOW), Ok(a), "the same again");
    assert_eq!(pins.resolve(b"lander.anyone", &builtin, Some(&moved), NOW), Err(Resolve::Changed));
    assert_eq!(pins.resolve(b"lander.anyone", &builtin, Some(&first), NOW), Ok(a), "and back is fine");
}

#[test]
fn resolution_says_why_not() {
    let builtin = defaults::list();
    let mut pins = Pins::default();
    let l = list(1, &[("lander.anyone", "a")]);
    assert_eq!(pins.resolve(b"lander.anyone", &builtin, None, NOW), Err(Resolve::Pending));
    let mut stale = l.clone();
    stale.valid_until = NOW - 1;
    assert_eq!(pins.resolve(b"lander.anyone", &builtin, Some(&stale), NOW), Err(Resolve::Pending), "an expired list is no list");
    assert_eq!(pins.resolve(b"other.anyone", &builtin, Some(&l), NOW), Err(Resolve::Unknown));
    assert_eq!(pins.resolve(target("a").as_bytes(), &builtin, Some(&l), NOW), Err(Resolve::NotAName));
    // The six built-in names resolve with no list at all, and a list cannot
    // move them.
    let hijack = list(9, &[("dns-live-1.anyone.anyone", "mallory")]);
    let live1 = parse(defaults::MAPPING[3].1.as_bytes()).unwrap();
    assert_eq!(pins.resolve(b"dns-live-1.anyone.anyone", &builtin, Some(&hijack), NOW), Ok(live1));
}

#[test]
fn the_pin_table_refuses_rather_than_forgets() {
    let builtin = defaults::list();
    let mut pins = Pins::default();
    let names: Vec<String> = (0..=PINS_MAX).map(|i| std::format!("n{i}.anyone")).collect();
    let mut entries: Vec<(Vec<u8>, [u8; 32])> = Vec::new();
    let id = parse(target("a").as_bytes()).unwrap();
    for n in &names {
        entries.push((n.clone().into_bytes(), id));
    }
    let l = NameList { published: 1, valid_until: NOW + 1, entries };
    for n in &names[..PINS_MAX] {
        assert!(pins.resolve(n.as_bytes(), &builtin, Some(&l), NOW).is_ok());
    }
    assert_eq!(pins.resolve(names[PINS_MAX].as_bytes(), &builtin, Some(&l), NOW), Err(Resolve::Full));
    assert!(pins.resolve(names[0].as_bytes(), &builtin, Some(&l), NOW).is_ok(), "pinned names still resolve");
}

#[test]
fn an_older_list_never_replaces_a_newer_one() {
    let newer = list(200, &[("x.anyone", "a")]);
    let older = list(100, &[("x.anyone", "a")]);
    assert!(may_replace(None, &older));
    assert!(may_replace(Some(&older), &newer));
    assert!(may_replace(Some(&newer), &newer), "the same list again");
    assert!(!may_replace(Some(&newer), &older));
}
