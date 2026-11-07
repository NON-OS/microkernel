// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! `ed25519 keygen`, `ed25519 sign SEED TEXT...` and
//! `ed25519 verify PUBLIC SIGNATURE TEXT...`: Ed25519 over the words of
//! TEXT joined by one space, as `echo` and `hash` read them. Keys and
//! signatures are lowercase hex. `keygen` draws its seed from the host's
//! generator and prints the seed and its public key; the seed is a secret
//! and anyone who reads the session can read it, so it is for test keys. The
//! key pair wipes itself when dropped.

use alloc::vec::Vec;

use super::hex::encode;
use crate::host::Host;
use crate::out::ShellOut;
use nonos_ed25519::{sign, verify, KeyPair, Signature};

const USAGE: &[u8] = b"usage: ed25519 keygen | sign SEED TEXT... | verify PUBLIC SIGNATURE TEXT...";

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host, argv: &[&[u8]]) -> bool {
    match argv.get(1..).unwrap_or(&[]) {
        [b"keygen"] => keygen(out, host),
        [b"sign", seed, text @ ..] => signed(out, seed, text),
        [b"verify", public, sig, text @ ..] => verified(out, public, sig, text),
        _ => {
            out.writeln(USAGE);
            false
        }
    }
}

fn keygen(out: &mut dyn ShellOut, host: &mut dyn Host) -> bool {
    let mut seed = [0u8; 32];
    if !host.random(&mut seed) {
        out.writeln(b"ed25519: the kernel would not draw randomness");
        return false;
    }
    let kp = KeyPair::from_seed(seed);
    seed.fill(0);
    out.writeln(&[&b"seed "[..], &encode(&kp.private)].concat());
    out.writeln(&[&b"public "[..], &encode(&kp.public)].concat());
    true
}

fn signed(out: &mut dyn ShellOut, seed: &[u8], text: &[&[u8]]) -> bool {
    let Some(mut s) = unhex::<32>(seed) else {
        out.writeln(b"ed25519: a seed is 64 hex digits");
        return false;
    };
    let kp = KeyPair::from_seed(s);
    s.fill(0);
    out.writeln(&encode(&sign(&kp, &join(text)).to_bytes()));
    true
}

fn verified(out: &mut dyn ShellOut, public: &[u8], sig: &[u8], text: &[&[u8]]) -> bool {
    let (Some(p), Some(s)) = (unhex::<32>(public), unhex::<64>(sig)) else {
        out.writeln(b"ed25519: a public key is 64 hex digits, a signature 128");
        return false;
    };
    let good = verify(&p, &join(text), &Signature::from_bytes(&s));
    out.writeln(if good { b"ed25519: good signature" } else { b"ed25519: bad signature" });
    good
}

fn join(words: &[&[u8]]) -> Vec<u8> {
    words.join(&b' ')
}

fn unhex<const N: usize>(s: &[u8]) -> Option<[u8; N]> {
    if s.len() != 2 * N {
        return None;
    }
    let mut out = [0u8; N];
    for (o, pair) in out.iter_mut().zip(s.chunks_exact(2)) {
        let digit = |c: u8| char::from(c).to_digit(16);
        *o = u8::try_from(digit(pair[0])? * 16 + digit(pair[1])?).ok()?;
    }
    Some(out)
}
