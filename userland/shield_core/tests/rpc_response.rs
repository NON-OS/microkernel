//! The JSON-RPC log parser against a real reply.
//!
//! The 107 KB `eth_getLogs` reply the Sepolia pool at 0xc0eEdE...D328 returned,
//! parsed the way the wallet parses a page. A reversed field or a mishandled
//! extra key is where a scanner silently finds nothing, so this runs the real
//! bytes and checks the 189 logs come out whole and feed the note decoder.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]

use nox_shield_core::discovery::{note_commitments, Log};
use nox_shield_core::net::rpc::{parse_block_number, parse_logs};

const REPLY: &str = include_str!("data/getlogs-c0eede-raw.json");

#[test]
fn the_parser_reads_every_log_from_the_real_reply() {
    let raw = parse_logs(REPLY).expect("a well formed reply parses");
    assert_eq!(raw.len(), 189, "every log in the page is read");
    // topics: event hash, commitment, leaf index. Data is empty for this event.
    assert_eq!(raw[0].topics.len(), 3);
    assert!(raw[0].data.is_empty(), "NoteCommitted carries no data");

    // The parsed logs feed the note decoder, the real path a scan takes.
    let logs: Vec<Log> = raw.iter().map(|r| Log { topics: &r.topics, data: &r.data }).collect();
    let map = note_commitments(&logs);
    assert_eq!(map.len(), 189, "every real leaf decodes from the parsed reply");
    assert!((0..189).all(|i| map.contains_key(&i)), "leaves 0..189");
}

#[test]
fn a_block_number_reply_parses() {
    let n = parse_block_number(r#"{"jsonrpc":"2.0","id":1,"result":"0xb355f2"}"#).expect("hex");
    assert_eq!(n, 0x00b3_55f2);
}

#[test]
fn an_error_reply_is_refused() {
    let reply = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32005,"message":"limit"}}"#;
    assert!(parse_logs(reply).is_err(), "a JSON-RPC error is not a page of logs");
}
