//! Reading a pasted or scanned receiving address, refusing anything that is not one.

use super::super::base32_read::decode;
use super::super::receive::ENCAPSULATION_KEY_BYTES;
use super::{checksum, CHECK, PREFIX, RECEIVING_ADDRESS_BYTES, RECEIVING_ADDRESS_VERSION};
use crate::error::WalletError;
use nonos_stark::field::P;

/// Read a pasted or scanned address. A bad checksum, spend word or encapsulation key is refused.
pub fn parse_receiving_address(
    text: &str,
) -> Result<([u64; 4], [u8; ENCAPSULATION_KEY_BYTES]), WalletError> {
    let cleaned: String =
        text.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase();
    let body = cleaned.strip_prefix(PREFIX).ok_or(WalletError::Address)?;
    let bytes = decode(body).ok_or(WalletError::Address)?;
    if bytes.len() != RECEIVING_ADDRESS_BYTES + CHECK {
        return Err(WalletError::Address);
    }
    let (a, check) = bytes.split_at(RECEIVING_ADDRESS_BYTES);
    if checksum(a)[..] != *check || a.first() != Some(&RECEIVING_ADDRESS_VERSION) {
        return Err(WalletError::Address);
    }
    let (keys, ek) = a.get(1..).ok_or(WalletError::Address)?.split_at(32);
    let mut spend_pk = [0u64; 4];
    for (w, chunk) in spend_pk.iter_mut().zip(keys.chunks_exact(8)) {
        let mut le = [0u8; 8];
        le.copy_from_slice(chunk);
        *w = u64::from_le_bytes(le);
        if *w >= P {
            return Err(WalletError::Address);
        }
    }
    let ek: [u8; ENCAPSULATION_KEY_BYTES] = ek.try_into().map_err(|_| WalletError::Address)?;
    x_wing::EncapsulationKey::try_from(ek.as_slice()).map_err(|_| WalletError::Address)?;
    Ok((spend_pk, ek))
}
