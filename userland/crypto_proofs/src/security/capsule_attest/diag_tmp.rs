use crate::crypto::zk_kernel::FieldElement;
fn hex(b: &[u8; 32]) -> String { b.iter().rev().map(|x| format!("{x:02x}")).collect() }
#[test]
fn diag() {
    let mut s = 0x1234_5678_9abc_def1u64;
    let mut next = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };
    for _ in 0..6 {
        let mut a = [0u8; 32]; let mut b = [0u8; 32];
        for i in 0..4 { a[i*8..i*8+8].copy_from_slice(&next().to_le_bytes()); b[i*8..i*8+8].copy_from_slice(&next().to_le_bytes()); }
        let (fa, fb) = (FieldElement::from_bytes(&a), FieldElement::from_bytes(&b));
        println!("CASE {} {} {} {} {} {}", hex(&a), hex(&b), hex(&fa.mul(&fb).to_bytes()), hex(&fa.add(&fb).to_bytes()), hex(&fa.to_bytes()), hex(&fb.to_bytes()));
    }
}
