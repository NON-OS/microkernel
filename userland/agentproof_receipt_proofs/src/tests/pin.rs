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

//! `agentproof-receipt pin`: a receipt's own AK, release and supervisor,
//! pinned only when the receipt verifies under them, so a pin is never given
//! for a receipt any other check refuses; and the file it writes reads back
//! as that trust and says it is first use, with no one vouching.

use agentproof_receipt::cli::answer;
use agentproof_receipt::cli_pin::{pinned, text};
use agentproof_receipt::trust_file::read;
use agentproof_receipt::verify::{pin, FIRST_USE};
use agentproof_receipt::Refusal;

use crate::fixture::Fixture;

#[test]
fn a_receipt_pins_its_own_trust_which_verifies_it() {
    let mut f = Fixture::new(b"strict");
    let (bytes, mut want) = (f.bytes(), f.trust());
    want.releases[0].version = FIRST_USE.into();
    let t = pin(&bytes).unwrap();
    assert_eq!(t, want);
    let a = pinned(&bytes);
    assert_eq!((a.code, a.err.len()), (0, 0));
    assert_eq!(read(&a.out[0]).unwrap(), t);
    assert!(a.out[0].contains("# Pinned on first use") && a.out[0].contains("No EK chain"));
    assert_eq!(answer(&bytes, &text(&t)).code, 0);
}

#[test]
fn a_receipt_any_other_check_refuses_pins_nothing() {
    let mut f = Fixture::new(b"strict");
    let bytes = f.bytes();
    let mut flipped = bytes.clone();
    flipped[30] ^= 1;
    assert_eq!(pin(&flipped), Err(Refusal::Challenge));
    assert_eq!(pinned(&flipped).code, 1);
    let mut f = Fixture::new(b"strict");
    f.spec.dma = (1, false, 0, 0);
    assert_eq!(pin(&f.bytes()), Err(Refusal::Dma));
    let mut f = Fixture::new(b"strict");
    f.record.flags = 0;
    assert_eq!(pin(&f.bytes()), Err(Refusal::Unfinished));
    let mut f = Fixture::new(b"strict");
    f.record.app_id = [3; 32];
    assert_eq!(pin(&f.bytes()), Err(Refusal::Binding));
}

#[test]
fn a_pin_trusts_no_other_receipt_signed_by_another_key() {
    let mut f = Fixture::new(b"strict");
    let t = pin(&f.bytes()).unwrap();
    let mut other = Fixture::new(b"strict");
    other.spec.doc_key ^= 1;
    other.spec.signing_key ^= 1;
    assert_eq!(answer(&other.bytes(), &text(&t)).code, 1);
}
