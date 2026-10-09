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

//! The four pinned production wallet vectors, embedded: STARKs'
//! spec/wallet-vectors-not-before at the commit the flake pins, copied under
//! vectors/ and held to it by the shield-vectors-pin flake check. The
//! production pool's verifier accepts these four proofs.

pub struct Vector {
    pub name: &'static str,
    pub request: &'static str,
    pub seed: &'static str,
    pub entropy: &'static str,
    /// nox_prover::to_json of the proof, byte for byte.
    pub proof_json: &'static [u8],
    /// The format 7 form the pool reads, byte for byte.
    pub format7: &'static [u8],
}

macro_rules! vector {
    ($name:literal) => {
        Vector {
            name: $name,
            request: include_str!(concat!("../vectors/", $name, "/request.json")),
            seed: include_str!(concat!("../vectors/", $name, "/seed.json")),
            entropy: include_str!(concat!("../vectors/", $name, "/entropy.hex")),
            proof_json: include_bytes!(concat!("../vectors/", $name, "/proof.json")),
            format7: include_bytes!(concat!("../vectors/", $name, "/proof-format7.bin")),
        }
    };
}

pub const ALL: [Vector; 4] = [
    vector!("transfer-eth"),
    vector!("withdraw-eth"),
    vector!("transfer-nox"),
    vector!("withdraw-nox"),
];

/// The entropy file's bytes; None unless it is exactly nox_prover's count.
pub fn entropy(v: &Vector) -> Option<Vec<u8>> {
    let hex = v.entropy.trim().trim_start_matches("0x");
    let digit = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let bytes: Option<Vec<u8>> = hex
        .as_bytes()
        .chunks(2)
        .map(|p| Some(digit(*p.first()?)? << 4 | digit(*p.get(1)?)?))
        .collect();
    bytes.filter(|b| b.len() == nox_prover::ENTROPY_BYTES)
}
