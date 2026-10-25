//! The nonce floor past a send a relay still holds: refused rather than
//! saturated, and never u64::MAX, which no transaction may carry (EIP-2681).

use crate::error::NetError;

/// `floor` raised past a held send at `nonce`, or ReplyShape when no nonce
/// is left past it.
pub(super) fn raise(floor: u64, nonce: u64) -> Result<u64, NetError> {
    let next = nonce.checked_add(1).filter(|n| *n < u64::MAX).ok_or(NetError::ReplyShape)?;
    Ok(floor.max(next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_held_send_raises_the_floor_and_never_saturates_onto_itself() {
        assert_eq!(raise(0, 4), Ok(5));
        assert_eq!(raise(9, 4), Ok(9), "a higher floor stands");
        assert_eq!(raise(0, u64::MAX - 2), Ok(u64::MAX - 1), "the last nonce a send may carry");
        assert_eq!(raise(0, u64::MAX - 1), Err(NetError::ReplyShape), "u64::MAX is no nonce");
        assert_eq!(raise(0, u64::MAX), Err(NetError::ReplyShape), "no wrap, no saturation");
    }
}
