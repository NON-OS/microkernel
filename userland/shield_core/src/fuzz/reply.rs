// NONOS Operating System (AGPL-3.0-or-later)
//! Rounds over the JSON-RPC reply readers: noise and near-misses built from the reply's own pieces.

use super::rng::Rng;
use crate::net::rpc::parse_logs;
use crate::net::rpc::{call_result, call_results, hex_bytes, objects, parse_block_number};

/// What a reply is made of, so the readers meet near-misses as well as noise.
const PIECES: [&str; 13] =
    ["{", "}", "[", "]", "\"result\"", "\"id\"", ":", ",", "\"0x", "\"", "\"error\"", "ff", "0"];

/// Run `rounds` rounds from `seed`; returns the number run.
pub fn reply_rounds(seed: u64, rounds: usize) -> usize {
    let mut rng = Rng::new(seed);
    for _ in 0..rounds {
        let text = hostile(&mut rng);
        held(&text, rng.below(6));
        let len = rng.below(80);
        let bytes = rng.bytes(len);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(hex_bytes(&format!("0x{hex}")), Some(bytes.clone()), "hex round trip");
        if !bytes.is_empty() {
            assert_eq!(hex_bytes(&format!("0x{hex}0")), None, "an odd digit count is refused");
            assert_eq!(hex_bytes(&hex), None, "hex without 0x is refused");
        }
    }
    rounds
}

/// A reply of pieces and noise, now and then far past any honest length.
pub(super) fn hostile(rng: &mut Rng) -> String {
    let len = match rng.below(16) {
        0 => 70_000,
        _ => rng.below(512),
    };
    let mut out = String::new();
    while out.len() < len {
        match rng.below(3) {
            0 => {
                let len = rng.below(8);
                out.push_str(&String::from_utf8_lossy(&rng.bytes(len)));
            }
            _ => out.push_str(PIECES[rng.below(PIECES.len())]),
        }
    }
    out
}

/// What every reader promises of any text: no panic, an error reply never read as an answer, a
/// batch read whole or not at all, and hex only when the whole text is hex.
fn held(text: &str, n: usize) {
    let errored = text.contains("\"error\"");
    if let Ok(answers) = call_results(text, n) {
        assert!(!errored && answers.len() == n, "a batch is read whole");
    }
    if errored {
        assert!(parse_block_number(text).is_err() && call_result(text).is_err());
    }
    if let Some(v) = hex_bytes(text) {
        assert_eq!(text.len(), 2 + 2 * v.len(), "hex is read only whole");
    }
    let _ = (parse_logs(text), objects(text));
}
