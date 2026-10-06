//! The calls the account makes, one constructor each. Every read is at the
//! latest block except the nonce, which counts transactions still pending so
//! a second send never reuses the first one's nonce.

use super::rpc::Call;

fn hex(bytes: &[u8]) -> String {
    let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!("0x{body}")
}

pub(super) fn balance(of: &[u8; 20]) -> Call {
    Call { method: "eth_getBalance", params: format!(r#"["{}","latest"]"#, hex(of)) }
}

pub(super) fn nonce(of: &[u8; 20]) -> Call {
    Call { method: "eth_getTransactionCount", params: format!(r#"["{}","pending"]"#, hex(of)) }
}

/// A read of `to`, made as `from` when one is given, which is how a transfer
/// is simulated as the sender before it is signed.
pub(super) fn call(from: Option<&[u8; 20]>, to: &[u8; 20], data: &[u8]) -> Call {
    let from = from.map(|f| format!(r#""from":"{}","#, hex(f))).unwrap_or_default();
    let params = format!(r#"[{{{from}"to":"{}","data":"{}"}},"latest"]"#, hex(to), hex(data));
    Call { method: "eth_call", params }
}

pub(super) fn estimate(from: &[u8; 20], to: &[u8; 20], value: u128, data: &[u8]) -> Call {
    let params = format!(
        r#"[{{"from":"{}","to":"{}","value":"0x{value:x}","data":"{}"}}]"#,
        hex(from),
        hex(to),
        hex(data)
    );
    Call { method: "eth_estimateGas", params }
}

pub(super) fn latest_block() -> Call {
    Call { method: "eth_getBlockByNumber", params: r#"["latest",false]"#.into() }
}

pub(super) fn priority_fee() -> Call {
    Call { method: "eth_maxPriorityFeePerGas", params: "[]".into() }
}

pub(super) fn storage(of: &[u8; 20], slot: &[u8; 32]) -> Call {
    Call {
        method: "eth_getStorageAt",
        params: format!(r#"["{}","{}","latest"]"#, hex(of), hex(slot)),
    }
}

pub(super) fn code(of: &[u8; 20]) -> Call {
    Call { method: "eth_getCode", params: format!(r#"["{}","latest"]"#, hex(of)) }
}

pub(super) fn send_raw(raw: &[u8]) -> Call {
    Call { method: "eth_sendRawTransaction", params: format!(r#"["{}"]"#, hex(raw)) }
}

pub(super) fn receipt(hash: &[u8; 32]) -> Call {
    Call { method: "eth_getTransactionReceipt", params: format!(r#"["{}"]"#, hex(hash)) }
}
