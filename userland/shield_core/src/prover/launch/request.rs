//! The request `nox_prover` proves against. Outputs are keyed by the payee's and this wallet's
//! spend keys, never "self", which the prover maps to a random key that loses the change.

use super::tree::word_hex;
use nonos_stark::air::RATE;

pub struct SpendRequest {
    pub pool_leaves: Vec<[u64; RATE]>,
    pub assoc_leaves: Vec<[u64; RATE]>,
    pub note_root: [u64; RATE],
    pub assoc_root: [u64; RATE],
    pub input_pool_index: [u64; 2],
    pub input_assoc_index: [u64; 2],
    /// Values of the two outputs: the payee's, then the change.
    pub output_values: [u64; 2],
    pub output_spend_pk: [[u64; RATE]; 2],
    /// The asset both inputs carry. The prover's standard sizes follow it.
    pub asset_id: u64,
    pub public_amount: u64,
    pub fee: u64,
    pub clearing_price: u64,
    pub recipient: [u8; 20],
    pub fee_recipient: [u8; 20],
    /// The earliest settlement time, a multiple of 600 seconds and never zero.
    pub not_before: u64,
}

impl SpendRequest {
    pub fn to_json(&self) -> String {
        let words = |xs: &[[u64; RATE]]| {
            xs.iter().map(|w| format!("\"{}\"", word_hex(w))).collect::<Vec<_>>().join(",")
        };
        let mut out = format!(
            "{{\"not_before\":{},\"pool_leaves\":[{}],\"assoc_leaves\":[{}],\"note_root\":\"{}\",\"assoc_root\":\"{}\",\
             \"input_pool_index\":[{},{}],\"input_assoc_index\":[{},{}],\"output_values\":[{},{}],\
             \"output_spend_pk\":[{}],\"public_amount\":{},\"fee\":{},\"clearing_price\":{},\
             \"recipient\":\"{}\",\"asset_id\":{}",
            self.not_before,
            words(&self.pool_leaves),
            words(&self.assoc_leaves),
            word_hex(&self.note_root),
            word_hex(&self.assoc_root),
            self.input_pool_index[0],
            self.input_pool_index[1],
            self.input_assoc_index[0],
            self.input_assoc_index[1],
            self.output_values[0],
            self.output_values[1],
            words(&self.output_spend_pk),
            self.public_amount,
            self.fee,
            self.clearing_price,
            address_hex(&self.recipient),
            self.asset_id,
        );
        // The prover refuses a fee recipient beside a zero fee.
        if self.fee != 0 {
            out.push_str(&format!(",\"fee_recipient\":\"{}\"", address_hex(&self.fee_recipient)));
        }
        out.push('}');
        out
    }
}

fn address_hex(a: &[u8; 20]) -> String {
    let mut s = String::from("0x");
    a.iter().for_each(|b| s.push_str(&format!("{b:02x}")));
    s
}
