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

//! Proving, verifying and sharing a spend at the production pool's point.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use stark_proofs::crypto::stark::air::{
    domain_params_blown, share_paths, stark_prove_ext_rounds, stark_prove_ext_rounds_top_observed,
    stark_verify_ext_rounds_positions, stark_verify_ext_rounds_shared_why,
    stark_verify_ext_rounds_why, WiredMultiGen,
};
use stark_proofs::crypto::stark::field::{Fp, P};
use stark_proofs::crypto::stark::fri::FRI_FOLD_LOG;
use stark_proofs::crypto::stark::fri_ext::QUERY_SHAPES;
use stark_proofs::crypto::stark::merkle::{TreeTop, DIGEST_BYTES};
use stark_proofs::proof_wire::{
    deserialize_rounds, deserialize_rounds_shared, read_header, serialize_rounds,
    serialize_rounds_shared, ParamSet,
};
use stark_proofs::recursion_assembly::inner::{hasher, hide_at};
use stark_proofs::shield::batch::assemble;
use stark_proofs::shield::join::publics::WORDS;
use stark_proofs::shield::join::{join_split_shape, JoinSplit};
use stark_proofs::shield::member::TREE_DEPTH;
use stark_proofs::shield_params::direct;
use stark_proofs::zk_rank::check_fri_rank;

use crate::json::Entropy;
use crate::spend::Created;

/// Random bytes a spend needs: the witness's output keys and blindings, and
/// the proof's own blinding.
pub const ENTROPY_BYTES: usize = 512;
/// How far down the periodic tree the cache keeps.
pub const PERIODIC_CUT: usize = 6;

/// The periodic tree's root for the production circuit (32-byte digests).
pub const PERIODIC_ROOT: [u8; 32] = [
    0x89, 0x8b, 0x80, 0x0f, 0x60, 0xf4, 0x67, 0xf0, 0x4a, 0xc9, 0x14, 0x0f, 0xb4, 0x25, 0xcd, 0x54,
    0x18, 0x1e, 0x4d, 0x16, 0x42, 0xf6, 0x1f, 0xc3, 0x8b, 0x59, 0x65, 0xd0, 0x8a, 0xce, 0x28, 0x88,
];
const _: () = assert!(DIGEST_BYTES == 32, "the production circuit keeps 32-byte digests");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Request(String),
    Policy(String),
    Entropy(String),
    Cache(String),
    Circuit(String),
    NotVerified(String),
    Rank(String),
    Cancelled,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Request(s) => write!(f, "request: {s}"),
            Error::Policy(s) => write!(f, "anonymity default: {s}"),
            Error::Entropy(s) => write!(f, "entropy: {s}"),
            Error::Cache(s) => write!(f, "periodic cache: {s}"),
            Error::Circuit(s) => write!(f, "circuit: {s}"),
            Error::NotVerified(s) => write!(f, "not verified: {s}"),
            Error::Rank(s) => write!(f, "rank condition: {s}"),
            Error::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Where a proof is, for a progress bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Request,
    Blinding,
    Region,
    Products,
    Periodic,
    Composition,
    CompositionTree,
    Deep,
    Fri,
    Queries,
    Verified,
}

impl Phase {
    fn of(name: &str) -> Option<(Phase, f32)> {
        Some(match name {
            "region committed" => (Phase::Region, 0.14),
            "products committed" => (Phase::Products, 0.22),
            "periodic" => (Phase::Periodic, 0.24),
            "composition" => (Phase::Composition, 0.42),
            "composition tree" => (Phase::CompositionTree, 0.46),
            "deep" => (Phase::Deep, 0.55),
            "fri" => (Phase::Fri, 0.93),
            "queries" => (Phase::Queries, 0.96),
            _ => return None,
        })
    }
}

/// What the caller hears while it proves, and how it stops one.
#[derive(Default)]
pub struct Progress<'a> {
    pub report: Option<&'a dyn Fn(Phase, f32)>,
    pub cancel: Option<&'a AtomicBool>,
}

impl Progress<'_> {
    fn report(&self, p: Phase, f: f32) {
        if let Some(cb) = self.report {
            cb(p, f);
        }
    }
    fn cancelled(&self) -> bool {
        self.cancel.map(|c| c.load(Ordering::Relaxed)).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rank {
    pub bound: usize,
    pub certified: usize,
    pub subsets: usize,
}

pub struct Proof {
    /// The proof in format 5, as the prover serialises it.
    pub bytes: Vec<u8>,
    /// The statement's public limbs.
    pub publics: Vec<u64>,
    pub created: Created,
    pub weakened: Vec<&'static str>,
    pub rank: Rank,
}

fn load_cache(bytes: &[u8]) -> Result<TreeTop, Error> {
    let top = TreeTop::from_bytes(bytes).ok_or_else(|| Error::Cache("malformed".into()))?;
    if top.root()[..] != PERIODIC_ROOT {
        return Err(Error::Cache("its root is not the production circuit's periodic root".into()));
    }
    Ok(top)
}

fn statement_at(
    publics: &[u64],
    q: usize,
    grind: u32,
) -> Result<(Vec<Fp>, WiredMultiGen, ParamSet), Error> {
    if publics.len() != WORDS {
        return Err(Error::Request(format!("{} public words, not {WORDS}", publics.len())));
    }
    if publics.iter().any(|&v| v >= P) {
        return Err(Error::Request("a public word is not below p".into()));
    }
    let words: Vec<Fp> = publics.iter().map(|&v| Fp::from_u64(v)).collect();
    let air = join_split_shape(TREE_DEPTH, &words);
    let params = ParamSet::of(&air, q, grind, direct::EXTRA_BLOWUP_BITS);
    Ok((words, air, params))
}

/// The query shape the proof's header names, among the accepted ones.
fn point_of(proof: &[u8], publics: &[u64]) -> Result<(usize, u32), Error> {
    let (_, air, _) = statement_at(publics, direct::N_QUERIES, direct::GRIND_BITS)?;
    let h = read_header(proof).ok_or_else(|| Error::NotVerified("no header".into()))?;
    for (_, q, g) in QUERY_SHAPES {
        if ParamSet::of(&air, q, g, direct::EXTRA_BLOWUP_BITS).id() == h.params {
            return Ok((q, g));
        }
    }
    Err(Error::NotVerified("the header names no accepted shape".into()))
}

/// Check a format 5 proof against the periodic root.
pub fn verify(proof: &[u8], publics: &[u64]) -> Result<(), Error> {
    let (q, grind) = point_of(proof, publics)?;
    let (words, air, params) = statement_at(publics, q, grind)?;
    let rounds = deserialize_rounds(proof, &params)
        .ok_or_else(|| Error::NotVerified("not a proof at the production point".into()))?;
    stark_verify_ext_rounds_why(
        air,
        &rounds,
        q,
        grind,
        direct::EXTRA_BLOWUP_BITS,
        &PERIODIC_ROOT,
        &words,
    )
    .map_err(|why| Error::NotVerified(why.to_string()))
}

/// The proof in format 7, the shared-path form the pool's verifier reads.
pub fn share(proof: &[u8], publics: &[u64]) -> Result<Vec<u8>, Error> {
    let (q, grind) = point_of(proof, publics)?;
    let (words, air, params) = statement_at(publics, q, grind)?;
    let extra = direct::EXTRA_BLOWUP_BITS;
    let log_n = domain_params_blown(&air, extra).0;
    let rounds = deserialize_rounds(proof, &params)
        .ok_or_else(|| Error::NotVerified("not a proof at the production point".into()))?;
    let positions =
        stark_verify_ext_rounds_positions(air, &rounds, q, grind, extra, &PERIODIC_ROOT, &words)
            .map_err(|why| Error::NotVerified(why.to_string()))?;
    let shared = share_paths(&rounds, &positions, log_n)
        .ok_or_else(|| Error::NotVerified("the proof's paths do not share".into()))?;
    Ok(serialize_rounds_shared(&rounds, &shared, &params))
}

/// Check a format 7 proof, as the pool will.
pub fn verify_shared(proof: &[u8], publics: &[u64]) -> Result<(), Error> {
    let (q, grind) = point_of(proof, publics)?;
    let (words, air, params) = statement_at(publics, q, grind)?;
    let (skeleton, shared) = deserialize_rounds_shared(proof, &params)
        .ok_or_else(|| Error::NotVerified("not a format 7 proof at the production point".into()))?;
    stark_verify_ext_rounds_shared_why(
        air,
        &skeleton,
        &shared,
        q,
        grind,
        direct::EXTRA_BLOWUP_BITS,
        &PERIODIC_ROOT,
        &words,
    )
    .map_err(|why| Error::NotVerified(why.to_string()))
}

/// Prove a spend from nothing. Returns the proof and the periodic cache,
/// which makes every later proof start a third faster.
pub fn prove(
    request: &str,
    seed: &str,
    entropy: &[u8],
    progress: &Progress<'_>,
) -> Result<(Proof, Vec<u8>), Error> {
    prove_inner(request, seed, entropy, None, progress)
}

/// Prove a spend from a periodic cache an earlier proof left.
pub fn prove_cached(
    request: &str,
    seed: &str,
    entropy: &[u8],
    cache: &[u8],
    progress: &Progress<'_>,
) -> Result<Proof, Error> {
    let top = load_cache(cache)?;
    prove_inner(request, seed, entropy, Some(&top), progress).map(|(p, _)| p)
}

fn prove_inner(
    request: &str,
    seed: &str,
    entropy: &[u8],
    top: Option<&TreeTop>,
    opts: &Progress<'_>,
) -> Result<(Proof, Vec<u8>), Error> {
    let weakened = crate::policy::check(request, &crate::policy::own_keys(seed))
        .map_err(Error::Policy)?;
    let (q, grind, extra) = (direct::N_QUERIES, direct::GRIND_BITS, direct::EXTRA_BLOWUP_BITS);
    let stream = crate::hedge::hedge(entropy, seed, request).map_err(Error::Entropy)?;
    let mut e = Entropy::new(&stream);
    let (parts, created) = {
        let mut words = |n: usize| e.words(n);
        crate::spend::build(request, seed, &mut words).map_err(Error::Request)?
    };
    let mut b = assemble(alloc::vec![parts]);
    let intent = b
        .intents
        .pop()
        .ok_or_else(|| Error::Circuit("the spend assembled no statement".into()))?;
    let mut js = JoinSplit { wired: b.wired, witness: b.witness, intent };
    opts.report(Phase::Request, 0.03);
    if opts.cancelled() {
        return Err(Error::Cancelled);
    }
    let h = hasher();
    let w = e.words(4).map_err(Error::Entropy)?;
    let s: [Fp; 4] = [w[0], w[1], w[2], w[3]];
    let blind = hide_at(&h, &mut js, &s, q, 1usize << FRI_FOLD_LOG);
    let params = ParamSet::of(&js.wired, q, grind, extra);
    let publics = js.intent.clone();
    let mut witness = js.witness;
    opts.report(Phase::Blinding, 0.05);
    let observe = |name: &'static str| -> bool {
        if let Some((p, f)) = Phase::of(name) {
            opts.report(p, f);
        }
        !opts.cancelled()
    };
    let (rounds, air, root, cache) = match top {
        Some(top) => {
            let proved = stark_prove_ext_rounds_top_observed(
                js.wired,
                &mut witness,
                q,
                grind,
                extra,
                &publics,
                top,
                &blind,
                &observe,
            );
            let Some((rounds, air)) = proved else {
                return Err(if opts.cancelled() {
                    Error::Cancelled
                } else {
                    Error::Cache("the periodic cache is not this circuit's".into())
                });
            };
            (rounds, air, top.root(), Vec::new())
        }
        None => {
            let (rounds, tree, air) = stark_prove_ext_rounds(
                js.wired,
                &mut witness,
                q,
                grind,
                extra,
                &publics,
                None,
                &blind,
            )
            .ok_or_else(|| {
                Error::Circuit("the circuit carries no permutation columns above its regions".into())
            })?;
            let cache = TreeTop::of(&tree, PERIODIC_CUT).map(|t| t.to_bytes()).unwrap_or_default();
            (rounds, air, tree.root(), cache)
        }
    };
    drop(witness);
    stark_verify_ext_rounds_why(air, &rounds, q, grind, extra, &root, &publics).map_err(|why| {
        Error::NotVerified(format!("the proof does not verify, nothing returned: {why}"))
    })?;
    opts.report(Phase::Verified, 1.0);
    let bytes = serialize_rounds(&rounds, &params);
    let words: Vec<u64> = publics.iter().map(|v| v.to_u64()).collect();
    // The rank condition on the bytes that leave, before they leave. A
    // shortfall returns nothing; the caller proves again with fresh entropy.
    let r = check_fri_rank(&bytes, &words, 3).map_err(Error::NotVerified)?;
    if !r.holds {
        return Err(Error::Rank(format!(
            "{} of {} certified after {} subsets; prove again with fresh entropy",
            r.mask_rank, r.bound, r.attempts
        )));
    }
    let rank = Rank { bound: r.bound, certified: r.mask_rank, subsets: r.attempts };
    Ok((Proof { bytes, publics: words, created, weakened, rank }, cache))
}
