// NONOS Operating System (AGPL-3.0-or-later)
//! Which units the driver programs, from their ID registers.

use crate::smmu::encode::idr::{decode, oas_bits, Features, Unusable};

/// QEMU's SMMUv3: stage 1, AArch64 tables, two-level stream table,
/// little-endian walks, coherent, 16-bit ASIDs; 16 SID bits, queues of
/// 2^19 entries; 4 KiB granule, 44-bit output.
const IDR0: u32 = 1 << 1 | 0b10 << 2 | 1 << 4 | 1 << 12 | 0b10 << 21 | 0b01 << 27;
const IDR1: u32 = 16 | 19 << 16 | 19 << 21;
const IDR5: u32 = 1 << 4 | 4;

#[test]
fn qemus_unit_is_usable() {
    let want = Features { sid_bits: 16, cmdq_log2: 19, eventq_log2: 19, oas_code: 4, asid16: true };
    assert_eq!(decode(IDR0, IDR1, IDR5), Ok(want));
    assert_eq!(oas_bits(want.oas_code), 44);
}

#[test]
fn each_missing_feature_is_named() {
    let without = |bits: u32| decode(IDR0 & !bits, IDR1, IDR5);
    assert_eq!(without(1 << 1), Err(Unusable::NoStage1));
    assert_eq!(without(0b10 << 2), Err(Unusable::NoAarch64Tables));
    assert_eq!(without(0b01 << 27), Err(Unusable::NoTwoLevelStreamTable));
    assert_eq!(without(1 << 4), Err(Unusable::NotCoherent));
    assert_eq!(decode(IDR0 | 0b11 << 21, IDR1, IDR5), Err(Unusable::NotLittleEndian));
    assert_eq!(decode(IDR0, IDR1, IDR5 & !(1 << 4)), Err(Unusable::No4kGranule));
    // Mixed-endian walks (0b00) are fine; so are both table formats.
    assert!(decode(IDR0 & !(0b11 << 21), IDR1, IDR5).is_ok());
    assert!(decode(IDR0 | 0b11 << 2, IDR1, IDR5).is_ok());
}

#[test]
fn oas_codes_and_the_reserved_ones() {
    let bits: Vec<u8> = (0..8).map(oas_bits).collect();
    assert_eq!(bits, [32, 36, 40, 42, 44, 48, 52, 52]);
    // A reserved OAS code is clamped to the largest defined one.
    assert_eq!(decode(IDR0, IDR1, (1 << 4) | 7).unwrap().oas_code, 6);
}
