// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware file (its TLVs, image loader and runtime sections, and the
//! split into LMAC, UMAC and paged images) and the PNVM file.

use crate::fuzz::Rng;
use crate::gen3::classify;
use crate::gen3::image::{find_iml, sections};
use crate::gen3::pnvm::{parse as pnvm, reserve};
use crate::gen3::ucode::Ucode;

use super::within;

const AX210: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
const ROUNDS: u32 = 2_000;

#[test]
fn a_firmware_file_yields_only_its_own_bytes() {
    let seed = &AX210[..64 * 1024];
    let mut r = Rng::seeded(0x0C0DE);
    for round in 0..ROUNDS {
        let blob = if round % 5 == 0 { r.noise(4096) } else { r.mutate(seed) };
        let _ = Ucode::parse(&blob);
        if let Some(iml) = find_iml(&blob) {
            assert!(within(&blob, iml), "round {round}");
        }
        assert!(sections(&blob).all(|s| within(&blob, s.data)), "round {round}");
        let l = classify(&blob);
        let all = l.lmac.iter().chain(&l.umac).chain(&l.virt);
        assert!(all.into_iter().all(|s| within(&blob, s.data)), "round {round}");
    }
}

#[test]
fn a_pnvm_file_yields_only_its_own_bytes() {
    let mut seed = Vec::new();
    for (kind, value) in
        [(64u32, vec![0xD1, 0x10, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0]), (58, vec![0x37, 0, 0x0D, 1])]
    {
        seed.extend_from_slice(&kind.to_le_bytes());
        seed.extend_from_slice(&(value.len() as u32).to_le_bytes());
        seed.extend_from_slice(&value);
    }
    let chunk = [&0u32.to_le_bytes()[..], &[0x5A; 60]].concat();
    seed.extend_from_slice(&19u32.to_le_bytes());
    seed.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
    seed.extend_from_slice(&chunk);
    assert!(pnvm(&seed, [0x0006_10D1, 0, 0], 0x37, 0x10D).is_some(), "the seed is a PNVM file");
    let mut r = Rng::seeded(0x9A7);
    for round in 0..ROUNDS * 10 {
        let f = if round % 4 == 0 { r.noise(512) } else { r.mutate(&seed) };
        let _ = pnvm(&f, [0x0006_10D1, 0, 0], 0x37, 0x10D);
        let _ = reserve(&f, 0x37, 0x10D);
    }
}
