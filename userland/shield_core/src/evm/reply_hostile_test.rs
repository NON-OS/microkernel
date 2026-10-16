// A test runs hostile input through each parser; any panic fails it.

//! A batch reply from an RPC host, read by every accessor: no panic and no loop.

use super::reply::{batch, one};

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
        r#"[{"id":0,"result":"0x"},{"id":0,"result":"0x1"}]"#.to_string(),
        r#"[{"id":99999999999999999999,"result":"0x1"}]"#.to_string(),
        r#"[{"id":1,"error":{"code":-32000,"message":"\"}}]"#.to_string(),
        r#"[{"id":0,"result":{"baseFeePerGas":"0x"}}]"#.to_string(),
        format!(r#"[{{"id":0,"result":"0x{}"}}]"#, "f".repeat(70)),
        format!(r#"[{{"id":0,"result":"0x{}"}}]"#, "0".repeat(100_000)),
    ]
    .to_vec()
}

#[test]
fn no_batch_reading_panics_on_hostile_replies() {
    for b in hostile(&misleading()) {
        let s = String::from_utf8_lossy(&b);
        for n in [0, 1, 3, 1000] {
            if let Ok(answers) = batch(&s, n) {
                for a in answers {
                    let _ = (a.quantity(), a.word(), a.data(), a.result());
                }
            }
        }
        if let Ok(a) = one(&s) {
            let _ = (a.quantity(), a.word(), a.data(), a.result());
        }
    }
}
