/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)]

use super::{batch, Answer};
use crate::error::NetError;

#[test]
fn a_batch_is_placed_by_id_whatever_order_it_arrives_in() {
    let reply =
        r#"[{"jsonrpc":"2.0","id":1,"result":"0x5208"},{"jsonrpc":"2.0","id":0,"result":"0x1"}]"#;
    let answers = batch(reply, 2).unwrap();
    assert_eq!(answers[0].quantity().unwrap(), 1);
    assert_eq!(answers[1].quantity().unwrap(), 21_000);
}

#[test]
fn a_revert_keeps_its_data_and_other_errors_fail_only_their_call() {
    let reverted = r#"[{"jsonrpc":"2.0","id":0,"error":{"code":3,"message":"execution reverted: g","data":"0x08c379a0"}}]"#;
    match &batch(reverted, 1).unwrap()[0] {
        Answer::Reverted(data) => assert_eq!(data, &[0x08, 0xc3, 0x79, 0xa0]),
        _ => panic!("a revert read as something else"),
    }
    let limited = r#"[{"jsonrpc":"2.0","id":0,"error":{"code":-32005,"message":"rate limited"}}]"#;
    let answers = batch(limited, 1).unwrap();
    assert!(matches!(answers[0], Answer::Failed));
    assert!(answers[0].quantity().is_err());
}

#[test]
fn a_missing_or_repeated_id_refuses_the_batch() {
    let one = r#"[{"jsonrpc":"2.0","id":0,"result":"0x1"}]"#;
    assert_eq!(batch(one, 2).err(), Some(NetError::ReplyShape));
    let twice = r#"[{"id":0,"result":"0x1"},{"id":0,"result":"0x2"}]"#;
    assert_eq!(batch(twice, 2).err(), Some(NetError::ReplyShape));
}

#[test]
fn a_quantity_wider_than_128_bits_is_refused() {
    let wide = format!(r#"[{{"id":0,"result":"0x1{}"}}]"#, "0".repeat(32));
    assert!(batch(&wide, 1).unwrap()[0].quantity().is_err());
}

#[test]
fn a_call_word_reads_as_a_number_and_an_oversized_one_is_refused() {
    let one = format!(r#"[{{"id":0,"result":"0x{}01"}}]"#, "0".repeat(62));
    assert_eq!(batch(&one, 1).unwrap()[0].word().unwrap(), 1);
    let huge = format!(r#"[{{"id":0,"result":"0x01{}"}}]"#, "0".repeat(62));
    assert!(batch(&huge, 1).unwrap()[0].word().is_err());
}
