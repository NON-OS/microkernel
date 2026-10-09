//! The account key at any index of m/44'/60'/0'/0/i, walked with the BIP-32 steps of nonos_hd.
//! Index 0 is the key `nonos_hd::derive_eth_key` gives, and the test holds the two together.
//! Every extended key wipes itself on drop, and on any failure the output stays zero.

use nonos_hd::bip32::{child_hardened, child_normal, compress_pubkey, master_from_seed, Xprv};
use zeroize::Zeroizing;

/// The first hardened index. An account index at or past it is not a normal step.
const HARDENED: u32 = 0x8000_0000;

/// The key at m/44'/60'/0'/0/`index`, or nothing for a hardened index or a step off the curve.
pub(super) fn key_at<F>(seed: &[u8; 64], index: u32, mut pubkey: F) -> Option<Zeroizing<[u8; 32]>>
where
    F: FnMut(&[u8; 32]) -> Option<[u8; 65]>,
{
    if index >= HARDENED {
        return None;
    }
    let master = master_from_seed(seed)?;
    let purpose = child_hardened(&master, 44)?;
    let coin = child_hardened(&purpose, 60)?;
    let account = child_hardened(&coin, 0)?;
    let change = normal(&account, &mut pubkey, 0)?;
    let leaf = normal(&change, &mut pubkey, index)?;
    Some(Zeroizing::new(leaf.key))
}

fn normal<F>(parent: &Xprv, pubkey: &mut F, index: u32) -> Option<Xprv>
where
    F: FnMut(&[u8; 32]) -> Option<[u8; 65]>,
{
    let uncompressed = Zeroizing::new(pubkey(&parent.key)?);
    let compressed = compress_pubkey(&uncompressed)?;
    child_normal(parent, &compressed, index)
}
