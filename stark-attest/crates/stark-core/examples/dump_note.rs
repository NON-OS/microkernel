//! Dumps the exact width-8 Poseidon note-commitment parameters and a reference
//! vector, so an independent implementation can be checked against it byte for
//! byte. Everything here comes from the vetted `air::poseidon` code; nothing is
//! recomputed by a different route except the MDS, which uses the same Cauchy
//! formula the permutation does and is printed so it can be compared.

use nonos_stark::air::{Poseidon, NOTE_DOMAIN, NOTE_LIMBS, RATE, WIDTH};
use nonos_stark::field::Fp;

fn main() {
    // log_t = 2 is what the note commitment uses: 2^2 = 4 full rounds.
    let p = Poseidon::new(2, [Fp::ZERO; RATE]);
    let rounds = 4usize;

    println!("P = 18446744069414584321  (Goldilocks)");
    println!("WIDTH = {WIDTH}  RATE = {RATE}  rounds = {rounds}  NOTE_DOMAIN = {NOTE_DOMAIN}");

    // Cauchy MDS: mds[i][j] = (i - (WIDTH + j))^{-1}. Same formula as cauchy_mds().
    println!("\nMDS:");
    for i in 0..WIDTH {
        let mut row = Vec::new();
        for j in 0..WIDTH {
            let x = Fp::from_u64(i as u64);
            let y = Fp::from_u64((WIDTH + j) as u64);
            row.push((x - y).inv().value().to_string());
        }
        println!("{}", row.join(","));
    }

    // Round constants, straight from the real schedule.
    println!("\nRC:");
    for r in 0..rounds {
        let rc = p.round_constant(r);
        let row: Vec<String> = rc.iter().map(|c| c.value().to_string()).collect();
        println!("{}", row.join(","));
    }

    // A reference note commitment over a fixed 11-limb preimage.
    let limbs_u: [u64; NOTE_LIMBS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let limbs: [Fp; NOTE_LIMBS] = core::array::from_fn(|i| Fp::from_u64(limbs_u[i]));
    let cm = p.commit_note(&limbs);
    println!("\nreference commit_note([1..=11]):");
    println!("{}", cm.iter().map(|c| c.value().to_string()).collect::<Vec<_>>().join(","));
}
