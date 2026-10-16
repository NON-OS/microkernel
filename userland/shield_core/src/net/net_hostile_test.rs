// A test runs hostile input through each parser; any panic fails it.

//! What an RPC host, a policy read or a fee read can send: no parser may panic or loop.

use super::fee_schedule::{bps, schedule};
use super::policy_paused::bool_word;
use super::policy_rules::decode_rule;
use super::rpc::{call_result, call_results, parse_block_number, parse_logs};

/// Bytes from a fixed seed, so a failure names the case that broke.
fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x.to_le_bytes()[0]
        })
        .collect()
}

/// Random bytes, runs of the characters a scanner keys on, and replies built to mislead.
fn hostile(misleading: &[String]) -> Vec<Vec<u8>> {
    let lens = (0..200usize).chain([1024, 70_000]);
    let random = lens.flat_map(|n| [bytes(n as u64 + 7, n), vec![b'{'; n], vec![b'"'; n]]);
    random.chain(misleading.iter().map(|s| s.clone().into_bytes())).collect()
}

fn misleading() -> Vec<String> {
    [
        r#"{"result":"0x"#.to_string(),
        r#"{"result":"0x1","result":"0x2"}"#.to_string(),
        r#"{"result":"0xzz"}"#.to_string(),
        r#"[{"id":1,"result":"0x"},{"id":1,"result":"0x1"}]"#.to_string(),
        r#"{"result":[{"topics":["0x"#.to_string(),
        r#"{"result":"\"0x1\""}"#.to_string(),
        "[{\"a\":".repeat(5_000),
        format!("{{\"result\":\"0x{}\"}}", "f".repeat(10_000)),
        format!("{{\"result\":\"0x{}\"}}", "1".repeat(64 * 300 + 1)),
    ]
    .to_vec()
}

#[test]
fn no_rpc_or_policy_parser_panics_on_hostile_replies() {
    for b in hostile(&misleading()) {
        let s = String::from_utf8_lossy(&b);
        let _ = (parse_block_number(&s), parse_logs(&s), call_result(&s), call_results(&s, 3));
        let _ = (call_results(&s, 0), call_results(&s, 64));
        let _ = (schedule(&b), bps(&b), bool_word(&b), decode_rule(&b));
    }
}
