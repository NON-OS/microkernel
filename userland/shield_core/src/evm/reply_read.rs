//! Reading one answer as the value it should be, refusing any other shape.

use super::reply::Answer;
use crate::error::NetError;
use crate::net::rpc::{hex_bytes, quoted};

impl Answer {
    /// A hex quantity, up to 128 bits: a balance, a fee, a nonce.
    pub(super) fn quantity(&self) -> Result<u128, NetError> {
        let hex = quoted(self.result()?).ok_or(NetError::ReplyShape)?;
        let body = hex.strip_prefix("0x").ok_or(NetError::ReplyShape)?;
        if body.is_empty() || body.len() > 32 {
            return Err(NetError::ReplyShape);
        }
        u128::from_str_radix(body, 16).map_err(|_| NetError::ReplyShape)
    }

    /// A call's single uint256 return word as 128 bits, refusing anything larger.
    pub(super) fn word(&self) -> Result<u128, NetError> {
        let data = self.data()?;
        let bytes: [u8; 32] = data.as_slice().try_into().map_err(|_| NetError::ReplyShape)?;
        let (high, low) = bytes.split_at(16);
        if high.iter().any(|b| *b != 0) {
            return Err(NetError::ReplyShape);
        }
        Ok(u128::from_be_bytes(low.try_into().map_err(|_| NetError::ReplyShape)?))
    }

    /// Hex data: what a call returned, a storage word, a transaction hash.
    pub(super) fn data(&self) -> Result<Vec<u8>, NetError> {
        hex_bytes(quoted(self.result()?).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)
    }

    /// The raw result text, for a result that is an object or null.
    pub(super) fn result(&self) -> Result<&str, NetError> {
        match self {
            Answer::Result(text) => Ok(text),
            Answer::Reverted(_) | Answer::Failed => Err(NetError::Rejected { code: 0 }),
        }
    }
}
