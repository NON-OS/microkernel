//! A member's stored bytes as its tar: decompressed when it is xz, within
//! the length its row gives, and taken only when it is the tar its row's
//! digest names. Pure, so the host proofs hold it.

use alloc::vec::Vec;

use nonos_app_image::{Codec, Member};

pub fn tar(member: &Member, stored: Vec<u8>) -> Option<Vec<u8>> {
    let tar = match member.codec {
        Codec::Tar => stored,
        Codec::Xz => {
            let out = nonos_xz::decompress(&stored)?;
            drop(stored);
            out
        }
    };
    member.holds(&tar).then_some(tar)
}
