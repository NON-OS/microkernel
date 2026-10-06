//! The read only JSON-RPC calls. Log queries filter by pool address and topic alone, so every
//! wallet sends the same request. A view tag filter would reveal the recipient set.
pub fn block_number() -> String {
    String::from(r#"{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}"#)
}

/// `eth_getLogs` for one event of one address over the caller's paging window.
pub fn get_logs(address: &str, topic0: &str, from_block: u64, to_block: u64) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"eth_getLogs","params":[{{"address":"{address}","topics":["{topic0}"],"fromBlock":"0x{from_block:x}","toBlock":"0x{to_block:x}"}}]}}"#
    )
}

/// `eth_call` of a view function at the latest block. Nothing here changes state.
pub fn eth_call(to: &str, data: &[u8]) -> String {
    let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"eth_call","params":[{{"to":"{to}","data":"0x{hex}"}},"latest"]}}"#
    )
}

/// Several `eth_call` reads as one batch, ids 0 to n-1: one request against the rate limit.
pub fn eth_call_batch(to: &str, datas: &[Vec<u8>]) -> String {
    let calls: Vec<String> = datas
        .iter()
        .enumerate()
        .map(|(id, data)| {
            let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
            format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"eth_call","params":[{{"to":"{to}","data":"0x{hex}"}},"latest"]}}"#)
        })
        .collect();
    format!("[{}]", calls.join(","))
}

/// A batch read at one block, so values match the state a log range ends at.
pub fn eth_call_batch_at(to: &str, datas: &[Vec<u8>], block: u64) -> String {
    eth_call_batch(to, datas).replace("\"latest\"", &format!("\"0x{block:x}\""))
}
