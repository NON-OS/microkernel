//! Reading a JSON-RPC reply from a machine the wallet does not control. A strict reader for one
//! shape, refusing an `error` member, bad hex, or a topic that is not 32 bytes.

use crate::error::NetError;

/// One log: indexed fields as topics, the rest as data, and the block it landed in.
#[derive(Clone)]
pub struct RawLog {
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub block: u64,
    /// The transaction that emitted it, when the server names one.
    pub tx: Option<[u8; 32]>,
}

/// The block number an `eth_blockNumber` reply carries.
pub fn block_number(reply: &str) -> Result<u64, NetError> {
    refuse_error(reply)?;
    let result = after(reply, "\"result\"").ok_or(NetError::ReplyShape)?;
    let hex = quoted(result).ok_or(NetError::ReplyShape)?;
    u64_hex(hex).ok_or(NetError::ReplyShape)
}

/// The `topics` and `data` of each log in an `eth_getLogs` reply.
pub fn logs(reply: &str) -> Result<Vec<RawLog>, NetError> {
    refuse_error(reply)?;
    let array = after(reply, "\"result\"").ok_or(NetError::ReplyShape)?;
    let mut out = Vec::new();
    for object in objects(array) {
        out.push(one_log(object)?);
    }
    Ok(out)
}

/// The bytes an `eth_call` returned.
pub fn call_result(reply: &str) -> Result<Vec<u8>, NetError> {
    refuse_error(reply)?;
    let result = after(reply, "\"result\"").ok_or(NetError::ReplyShape)?;
    hex_bytes(quoted(result).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)
}

/// A batch of `n` results placed by id. An error, a missing id or a repeat refuses the batch.
pub fn call_results(reply: &str, n: usize) -> Result<Vec<Vec<u8>>, NetError> {
    refuse_error(reply)?;
    let mut out: Vec<Option<Vec<u8>>> = vec![None; n];
    for object in objects(reply) {
        let id_at = after(object, "\"id\"").ok_or(NetError::ReplyShape)?;
        let id_text: String = id_at
            .trim_start_matches(|c: char| c == ':' || c.is_whitespace())
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        let id: usize = id_text.parse().map_err(|_| NetError::ReplyShape)?;
        let result = after(object, "\"result\"").ok_or(NetError::ReplyShape)?;
        let bytes =
            hex_bytes(quoted(result).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)?;
        let slot = out.get_mut(id).ok_or(NetError::ReplyShape)?;
        if slot.replace(bytes).is_some() {
            return Err(NetError::ReplyShape);
        }
    }
    out.into_iter().map(|r| r.ok_or(NetError::ReplyShape)).collect()
}

fn one_log(object: &str) -> Result<RawLog, NetError> {
    let topics_at = after(object, "\"topics\"").ok_or(NetError::ReplyShape)?;
    let mut topics = Vec::new();
    for hex in strings_in_array(topics_at).ok_or(NetError::ReplyShape)? {
        topics.push(bytes32(hex).ok_or(NetError::ReplyShape)?);
    }
    let data_at = after(object, "\"data\"").ok_or(NetError::ReplyShape)?;
    let data =
        hex_bytes(quoted(data_at).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)?;
    let block_at = after(object, "\"blockNumber\"").ok_or(NetError::ReplyShape)?;
    let block =
        u64_hex(quoted(block_at).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)?;
    let tx = after(object, "\"transactionHash\"").and_then(quoted).and_then(bytes32);
    Ok(RawLog { topics, data, block, tx })
}

/// Refuse a reply carrying a JSON-RPC error.
fn refuse_error(reply: &str) -> Result<(), NetError> {
    if reply.contains("\"error\"") {
        return Err(NetError::ReplyShape);
    }
    Ok(())
}

/// The slice just past a key, or nothing if the key is absent.
pub(crate) fn after<'a>(s: &'a str, key: &str) -> Option<&'a str> {
    let at = s.find(key)?;
    s.get(at.checked_add(key.len())?..)
}

/// The first double-quoted run in a slice, without the quotes.
pub(crate) fn quoted(s: &str) -> Option<&str> {
    let open = s.find('"')?;
    let rest = s.get(open.checked_add(1)?..)?;
    let close = rest.find('"')?;
    rest.get(..close)
}

/// The top level objects of the first array in a slice, split on brace depth
/// with string literals respected so a brace inside a string does not count.
pub(crate) fn objects(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let (mut depth, mut start, mut in_str, mut esc, mut seen_array) =
        (0i32, 0usize, false, false, false);
    for (i, &b) in bytes.iter().enumerate() {
        if in_str {
            if esc {
                esc = false;
            } else if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_str = false;
            }
            continue;
        }
        match b {
            b'"' => in_str = true,
            b'[' if !seen_array => seen_array = true,
            b'{' => {
                if depth == 0 {
                    start = i;
                }
                depth = depth.saturating_add(1);
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    if let Some(slice) = s.get(start..=i) {
                        out.push(slice);
                    }
                }
            }
            b']' if depth == 0 && seen_array => break,
            _ => {}
        }
    }
    out
}

/// The double-quoted strings of the first bracketed array in a slice.
fn strings_in_array(s: &str) -> Option<Vec<&str>> {
    let open = s.find('[')?;
    let rest = s.get(open.checked_add(1)?..)?;
    let close = rest.find(']')?;
    let inner = rest.get(..close)?;
    let mut out = Vec::new();
    let mut cursor = inner;
    while let Some(q) = quoted(cursor) {
        out.push(q);
        let past = cursor.find(q)?.checked_add(q.len())?.checked_add(1)?;
        cursor = cursor.get(past..)?;
    }
    Some(out)
}

/// A `0x` prefixed 32 byte word.
fn bytes32(hex: &str) -> Option<[u8; 32]> {
    let bytes = hex_bytes(hex)?;
    <[u8; 32]>::try_from(bytes.as_slice()).ok()
}

/// The bytes a `0x` prefixed hex string stands for, empty for `0x`.
pub(crate) fn hex_bytes(hex: &str) -> Option<Vec<u8>> {
    let body = hex.strip_prefix("0x")?;
    if !body.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::new();
    for pair in body.as_bytes().chunks_exact(2) {
        let hi = (char::from(*pair.first()?)).to_digit(16)?;
        let lo = (char::from(*pair.get(1)?)).to_digit(16)?;
        out.push(u8::try_from(hi.checked_shl(4)?.checked_add(lo)?).ok()?);
    }
    Some(out)
}

/// A `0x` prefixed hex number as a `u64`.
fn u64_hex(hex: &str) -> Option<u64> {
    let body = hex.strip_prefix("0x")?;
    u64::from_str_radix(body, 16).ok()
}
