//! Signatures over a link digest, made with a key this wallet holds or brought from the wallet of
//! the mainnet address: 65 bytes, r, s in the lower half, then v of 27 or 28.

use k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey};
use sha3::{Digest, Keccak256};

/// `key` over `digest`, in the form the registry reads.
pub fn sign_link(key: &SigningKey, digest: &[u8; 32]) -> Option<[u8; 65]> {
    let (signature, recovery) = key.sign_prehash_recoverable(digest).ok()?;
    let mut out = [0u8; 65];
    let (rs, v) = out.split_at_mut(64);
    rs.copy_from_slice(&signature.to_bytes());
    *v.first_mut()? = 27u8.checked_add(u8::from(recovery.is_y_odd()))?;
    Some(out)
}

/// A signature pasted as `0x` hex, its v of 0, 1, 27 or 28 written as 27 or 28, or none when the
/// registry would refuse it.
pub fn parse_signature(text: &str) -> Option<[u8; 65]> {
    let bytes = crate::net::rpc::hex_bytes(text.trim())?;
    let mut out: [u8; 65] = bytes.as_slice().try_into().ok()?;
    let v = out.get_mut(64)?;
    *v = match *v {
        0 | 27 => 27,
        1 | 28 => 28,
        _ => return None,
    };
    let signature = Signature::from_slice(out.get(..64)?).ok()?;
    signature.normalize_s().is_none().then_some(out)
}

/// The address whose key signed `digest`, or none.
pub fn signer(digest: &[u8; 32], signature: &[u8; 65]) -> Option<[u8; 20]> {
    let (rs, v) = signature.split_at(64);
    let recovery = RecoveryId::from_byte(v.first()?.checked_sub(27)?)?;
    let rs = Signature::from_slice(rs).ok()?;
    let key = VerifyingKey::recover_from_prehash(digest, &rs, recovery).ok()?;
    let point = key.to_encoded_point(false);
    let hash = Keccak256::digest(point.as_bytes().get(1..)?);
    hash.get(12..)?.try_into().ok()
}
