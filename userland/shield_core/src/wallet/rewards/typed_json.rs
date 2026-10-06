//! The link message as the typed data a wallet that holds the mainnet address signs with
//! `eth_signTypedData_v4`, for a mainnet address this wallet does not hold.

use super::typed::{CHAIN_ID, NAME, VERSION};
use crate::evm::checksummed;

/// The typed data `mainnet` signs to link itself to `testnet` at `nonce`.
pub fn typed_json(mainnet: &[u8; 20], testnet: &[u8; 20], nonce: u128) -> String {
    let field = |name: &str, kind: &str| format!(r#"{{"name":"{name}","type":"{kind}"}}"#);
    let domain_fields =
        [field("name", "string"), field("version", "string"), field("chainId", "uint256")];
    let link_fields =
        [field("mainnet", "address"), field("testnet", "address"), field("nonce", "uint256")];
    format!(
        concat!(
            r#"{{"types":{{"EIP712Domain":[{}],"Link":[{}]}},"primaryType":"Link","#,
            r#""domain":{{"name":"{}","version":"{}","chainId":{}}},"#,
            r#""message":{{"mainnet":"{}","testnet":"{}","nonce":"{}"}}}}"#
        ),
        domain_fields.join(","),
        link_fields.join(","),
        NAME,
        VERSION,
        CHAIN_ID,
        checksummed(mainnet),
        checksummed(testnet),
        nonce
    )
}
