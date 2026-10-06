/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

//! A vector for every refusal of a view key, each built from account 0 of the test phrase with
//! one thing wrong and the checksum made right again where the fault is not the checksum.

use super::accounts_test::seed;
use super::base32::encode;
use super::view_key::{body, checksum, padded, ViewKind, CHECK, FULL, INCOMING};
use super::view_key_read::parse_view_key;
use super::view_key_text::view_key_text;
use super::Account;
use nonos_stark::field::P;

/// The key bytes of account 0 before the checksum.
fn key(kind: ViewKind) -> Vec<u8> {
    let text = view_key_text(&Account::at(&seed(), 0).unwrap(), kind);
    let prefix = if kind == ViewKind::Full { FULL } else { INCOMING };
    let all = super::base32_read::decode(&text[prefix.len()..]).unwrap();
    all[..body(kind)].to_vec()
}

/// The text of `bytes` under `prefix`, checksummed as `kind` and padded, with `pad` as last byte.
pub(super) fn text(prefix: &str, kind: ViewKind, bytes: &[u8], pad: u8) -> String {
    let mut all = bytes.to_vec();
    all.extend_from_slice(&checksum(kind, bytes));
    all.resize(padded(kind), 0);
    if all.len() > body(kind) + CHECK {
        *all.last_mut().unwrap() = pad;
    }
    format!("{prefix}{}", encode(&all).unwrap())
}

/// Every refusal, by name, with the text that must be refused.
pub(super) fn refusals() -> Vec<(&'static str, String)> {
    let (ivk, fvk) = (key(ViewKind::Incoming), key(ViewKind::Full));
    let good = text(INCOMING, ViewKind::Incoming, &ivk, 0);
    let mut sum = ivk.clone();
    sum.extend_from_slice(&checksum(ViewKind::Incoming, &ivk));
    sum[body(ViewKind::Incoming)] ^= 1;
    sum.resize(padded(ViewKind::Incoming), 0);
    let bad_check = format!("{INCOMING}{}", encode(&sum).unwrap());
    let mut version = ivk.clone();
    version[0] = 0x02;
    let mut spend = ivk.clone();
    spend[33..41].copy_from_slice(&P.to_le_bytes());
    let mut nk = fvk.clone();
    nk[65..73].copy_from_slice(&P.to_le_bytes());
    vec![
        ("wrong checksum", bad_check),
        ("wrong version", text(INCOMING, ViewKind::Incoming, &version, 0)),
        ("relabelled prefix", format!("{FULL}{}", &good[INCOMING.len()..])),
        ("unknown prefix", format!("noxxvk1{}", &good[INCOMING.len()..])),
        ("non-zero pad", text(FULL, ViewKind::Full, &fvk, 1)),
        ("spend_pk word at p", text(INCOMING, ViewKind::Incoming, &spend, 0)),
        ("nk word at p", text(FULL, ViewKind::Full, &nk, 0)),
    ]
}

#[test]
fn the_unbroken_key_reads_so_each_refusal_is_its_one_fault() {
    let ivk = key(ViewKind::Incoming);
    assert!(parse_view_key(&text(INCOMING, ViewKind::Incoming, &ivk, 0)).is_ok());
    assert!(parse_view_key(&text(FULL, ViewKind::Full, &key(ViewKind::Full), 0)).is_ok());
}

#[test]
fn every_refusal_is_refused() {
    for (name, bad) in refusals() {
        assert!(parse_view_key(&bad).is_err(), "{name} was accepted");
    }
}
