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

//! A proof the kernel accepted is answered again within its family, for the
//! very bytes it was given and nothing else, and every family starts with no
//! answers: each guest family's seal still records what its guests ran under.

#[path = "../../../capsule_linux/src/linux/attest_seen_key.rs"]
mod seen_key;
#[path = "../../../capsule_linux/src/linux/attest_seen_rule.rs"]
mod rule;

use rule::{Seen, MAX_SEEN};
use seen_key::key;

const SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_linux/src");
const ATTEST: &str = include_str!("../../../capsule_linux/src/linux/attest.rs");

#[test]
fn every_byte_the_kernel_reads_is_in_the_key() {
    let (img, tr, cert, man) = (&b"image"[..], &b"trailer"[..], &b"cert"[..], &b"manifest"[..]);
    let base = key(img, tr, Some((cert, man)));
    assert_eq!(base, key(img, tr, Some((cert, man))), "the same bytes, the same key");
    assert_ne!(base, key(b"imagE", tr, Some((cert, man))), "the image");
    assert_ne!(base, key(img, b"trailEr", Some((cert, man))), "the trailer");
    assert_ne!(base, key(img, tr, Some((b"cerT", man))), "the certificate");
    assert_ne!(base, key(img, tr, Some((cert, b"manifesT"))), "the manifest");
    assert_ne!(key(img, tr, None), key(img, tr, Some((b"", b""))), "local is not publisher");
}

#[test]
fn bytes_moved_between_fields_are_another_key() {
    assert_ne!(key(b"ab", b"c", None), key(b"a", b"bc", None));
    assert_ne!(key(b"x", b"t", Some((b"cm", b""))), key(b"x", b"t", Some((b"c", b"m"))));
}

#[test]
fn only_an_acceptance_is_answered_and_only_in_its_family() {
    let mut seen: Seen<u32> = Seen::new();
    let (a, b) = (key(b"one", b"t", None), key(b"two", b"t", None));
    assert_eq!(seen.known(&a), None, "nothing is known before the kernel said so");
    seen.accepted(a, 7);
    assert_eq!(seen.known(&a), Some(7));
    assert_eq!(seen.known(&b), None, "other bytes are asked again");
    seen.accepted(a, 9);
    assert_eq!(seen.known(&a), Some(7), "the first answer stands");
    seen.forget();
    assert_eq!(seen.known(&a), None, "a new family asks the kernel itself");
}

#[test]
fn a_full_table_keeps_what_it_has_and_asks_the_kernel_for_the_rest() {
    let mut seen: Seen<usize> = Seen::new();
    let keys: Vec<[u8; 32]> =
        (0..=MAX_SEEN).map(|i| key(&i.to_le_bytes(), b"trailer", None)).collect();
    for (i, k) in keys.iter().enumerate() {
        seen.accepted(*k, i);
    }
    assert_eq!(seen.known(&keys[0]), Some(0));
    assert_eq!(seen.known(&keys[MAX_SEEN - 1]), Some(MAX_SEEN - 1));
    assert_eq!(seen.known(&keys[MAX_SEEN]), None, "past the bound, nothing is kept");
}

#[test]
fn the_kernel_is_asked_unless_this_family_had_it_accept_these_bytes() {
    let body = &ATTEST[ATTEST.find("pub fn verify(").expect("verify is gone")..];
    let at = |needle: &str| body.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let keyed = at("attest_seen::key(image, &trailer, held)");
    let known = at("if let Some(summary) = attest_seen::known(&seen) {");
    let local = at("attest_local::verify(image, &trailer)?");
    let publisher = at("attest_publisher::verify(image, cert, manifest, &trailer)?");
    let accepted = at("attest_seen::accepted(seen, summary);");
    assert!(keyed < known && known < local.min(publisher) && local.max(publisher) < accepted);
}

/// Every guest a family can begin with is spawned after the answers are
/// forgotten, in the same few lines, with nothing proved between.
#[test]
fn every_spawn_that_may_begin_a_family_forgets_first() {
    let mut files = vec![std::path::PathBuf::from(SRC)];
    let mut spawns = 0;
    while let Some(at) = files.pop() {
        if at.is_dir() {
            files.extend(std::fs::read_dir(&at).unwrap().map(|e| e.unwrap().path()));
            continue;
        }
        if at.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let text = std::fs::read_to_string(&at).unwrap();
        for (i, _) in text.match_indices("mk_foreign_spawn(b\"") {
            spawns += 1;
            let before = &text[..i];
            let forgot = before.rfind("attest_seen::forget();").unwrap_or_else(|| {
                panic!("{} spawns a guest without forgetting the last family's", at.display())
            });
            let between = &before[forgot..];
            assert!(between.lines().count() <= 4, "{}: forget is not next to it", at.display());
            assert!(!between.contains("verify") && !between.contains("prove"));
        }
    }
    assert_eq!(spawns, 2, "the personality's guest and GTK's tools: a new spawn site needs a look");
}

const MAP_EXEC: &str = include_str!("../../../capsule_linux/src/linux/call/mem/map_exec.rs");
const MAP_FILE: &str = include_str!("../../../capsule_linux/src/linux/call/mem/map_file.rs");
const MAP_WHOLE: &str = include_str!("../../../capsule_linux/src/linux/call/mem/map_whole.rs");

/// The bytes mapped executable are the bytes proved on this ask, read once:
/// from the descriptor's stream when it has them whole, else from the store.
#[test]
fn an_exec_mapping_is_filled_from_the_bytes_proved_on_this_ask() {
    let body = &MAP_EXEC[MAP_EXEC.find("pub fn proven(").expect("proven is gone")..];
    let at = |needle: &str| body.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let read = at("let bytes = match whole(entry, MAX_IMAGE) {");
    let fallback = at("None => store_read(&key(&at), MAX_IMAGE).ok()?,");
    let proved = at("crate::linux::attest::verify(&at, &bytes).ok()?;");
    let given = at("Some(bytes)\n}");
    assert!(read < fallback && fallback < proved && proved < given);
    assert!(MAP_FILE.contains("if let Some(Some(bytes)) = proved {"));
    assert!(MAP_FILE.contains("fill_from(guest, &bytes, req, at)"), "filled from the proved bytes");
    let whole = &MAP_WHOLE[MAP_WHOLE.find("pub fn whole(").expect("whole is gone")..];
    assert!(whole.contains("entry.writable || !entry.pending.is_empty()"), "a file being written");
    assert!(whole.contains("(bytes.len() as u64 == size).then_some(bytes)"), "a cut read");
}
