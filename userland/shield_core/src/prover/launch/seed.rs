//! The seed file of a spend for the prover: the two secret keys and the two notes they open. Both
//! inputs belong to this wallet, so both secrets are its spend secret, and a single note pairs with
//! a dummy worth zero, which the circuit leaves out of every tree.

use crate::notes::NotePlaintext;
use core::fmt::Write;
use nonos_stark::air::RATE;
use zeroize::Zeroizing;

/// Room for the longest seed file, so the buffer that holds the secret never grows behind it.
const ROOM: usize = 1024;

/// The seed file for spending `inputs` under `secret`. It holds the secret, so it is written into
/// one buffer with no copy along the way, and wiped when dropped.
pub fn seed_json(secret: &[u64; RATE], inputs: [&NotePlaintext; 2]) -> Zeroizing<String> {
    let mut out = Zeroizing::new(String::with_capacity(ROOM));
    let _ = write_seed(&mut out, secret, inputs);
    out
}

fn quad(out: &mut String, w: &[u64; RATE]) -> core::fmt::Result {
    write!(out, "[{},{},{},{}]", w[0], w[1], w[2], w[3])
}

fn write_seed(
    out: &mut String,
    secret: &[u64; RATE],
    inputs: [&NotePlaintext; 2],
) -> core::fmt::Result {
    out.push_str("{\"secrets\":[");
    quad(out, secret)?;
    out.push(',');
    quad(out, secret)?;
    out.push_str("],\"notes\":[");
    for (i, n) in inputs.iter().enumerate() {
        out.push_str(if i == 0 { "" } else { "," });
        write!(out, "{{\"value\":{},\"asset_id\":{},\"spend_pk\":", n.value, n.asset_id)?;
        quad(out, &n.spend_pk)?;
        out.push_str(",\"blinding\":");
        quad(out, &n.blinding)?;
        out.push('}');
    }
    out.push_str("]}");
    Ok(())
}

#[cfg(test)]
#[path = "seed_test.rs"]
mod seed_test;
