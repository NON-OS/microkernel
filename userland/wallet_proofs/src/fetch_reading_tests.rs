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

//! A reading that did not come, a receipt for another transaction, a fee refused, and an answer cut mid-record.

#[test]
fn a_reading_that_did_not_come_is_not_read_not_the_last_value() {
    use crate::reading::take;
    let (mut value, mut ready) = (0u64, false);
    assert!(take(Some(7), &mut value, &mut ready), "a first reading is shown");
    assert!((value, ready) == (7, true));
    assert!(!take(Some(7), &mut value, &mut ready), "the same reading changes nothing");
    assert!(take(None, &mut value, &mut ready), "a missing reading changes what is shown");
    assert!(!ready, "and is shown as not read");
    assert!(!take(None, &mut value, &mut ready), "still not read");
    assert!(take(Some(7), &mut value, &mut ready), "the same value read again is shown again");
}

#[test]
fn a_receipt_counts_only_for_the_transaction_it_names() {
    use crate::wallet::rpc::parse_receipt_for;
    let hash = [0xab; 32];
    let ours = format!(
        r#"{{"id":6,"result":{{"transactionHash":"0x{}","status":"0x1"}}}}"#,
        "AB".repeat(32)
    );
    assert_eq!(parse_receipt_for(ours.as_bytes(), &hash), Some(true), "capitals or not");
    let other = format!(
        r#"{{"id":6,"result":{{"transactionHash":"0x{}","status":"0x1"}}}}"#,
        "cd".repeat(32)
    );
    assert_eq!(parse_receipt_for(other.as_bytes(), &hash), None, "another transaction's receipt");
    assert_eq!(parse_receipt_for(br#"{"id":6,"result":null}"#, &hash), None);
}

#[test]
fn a_zero_or_missing_fee_is_refused_with_its_own_reason() {
    use crate::wallet::send::gas::fee_refusal;
    assert_ne!(fee_refusal(None), fee_refusal(Some(0)));
    assert!(fee_refusal(Some(1_000_000_000)).is_none());
}

#[test]
fn an_answer_cut_mid_record_is_told_from_one_that_fails_to_open() {
    use crate::records_whole::records_whole;
    let record = |len: usize| {
        let mut r = vec![23u8, 3, 3, (len >> 8) as u8, len as u8];
        r.extend(core::iter::repeat_n(0xAAu8, len));
        r
    };
    let mut two = record(40);
    two.extend(record(300));
    assert!(records_whole(&two), "two whole records");
    assert!(records_whole(&[]), "nothing is a run of no records");
    assert!(!records_whole(&two[..two.len() - 1]), "a record one byte short");
    assert!(!records_whole(&two[..3]), "a header cut short");
    let mut trailing = two.clone();
    trailing.push(23);
    assert!(!records_whole(&trailing), "a byte past the last record");
}
