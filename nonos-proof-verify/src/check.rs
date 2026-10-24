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

//! The check itself: the four steps every gate runs, in the gates' order, with
//! the gates' own crates. Nothing here proves or folds on its own.

use nonos_attest_path::{parse_v4, root_words, verify, Kind, MAGIC};
use nox_verify::statements::ATTEST;

/// The depth every gate folds to (the kernel's POLICY_TREE_DEPTH, the enroll
/// tool's DEPTH).
pub const DEPTH: usize = 8;

/// What a trailer that passed carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proof {
    /// A v4 trailer: the path folds to the root and the STARK verifies.
    Stark,
    /// A development trailer: the path alone, no STARK. Only a development
    /// image's gates take it; a release refuses it.
    PathOnly,
}

/// Why a trailer was refused. Each has a stable stage name and code, for a
/// log or a script; the codes follow the boot gate's STARK band for the
/// verifier's own refusals (400 plus `Refusal::code`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// Not a v4 trailer of the expected kind: magic, kind byte, a length out
    /// of bound, an inner path that is not a v3 path, or bytes past the proof.
    Malformed,
    /// A development trailer (the path alone), and development trailers were
    /// not allowed.
    PathOnlyRefused,
    /// The path does not fold to the root for this context.
    Path,
    /// The root is not four canonical field words, so no proof can bind it.
    Root,
    /// The kind has no public words in the attestation statement.
    Publics,
    /// The STARK verifier refused the proof.
    Stark(nox_verify::Refusal, &'static str),
}

impl Refused {
    pub fn stage(self) -> &'static str {
        match self {
            Refused::Malformed => "parse",
            Refused::PathOnlyRefused => "development",
            Refused::Path => "path",
            Refused::Root => "root",
            Refused::Publics => "publics",
            Refused::Stark(..) => "stark",
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Refused::Malformed => 300,
            Refused::PathOnlyRefused => 301,
            Refused::Path => 302,
            Refused::Root => 303,
            Refused::Publics => 304,
            Refused::Stark(r, _) => 400 + r.code(),
        }
    }

    pub fn reason(self) -> String {
        match self {
            Refused::Malformed => "not a v4 trailer of this kind".into(),
            Refused::PathOnlyRefused => {
                "a development trailer (path only, no STARK); a release refuses it".into()
            }
            Refused::Path => "the path does not fold to the root for this measurement".into(),
            Refused::Root => "the root is not four canonical field words".into(),
            Refused::Publics => "the statement has no public words for this kind".into(),
            Refused::Stark(r, why) if why != r.reason() => {
                format!("the STARK was refused: {} ({why})", r.reason())
            }
            Refused::Stark(r, _) => format!("the STARK was refused: {}", r.reason()),
        }
    }
}

/// The statement's kind number for a leaf kind, as the gates name it. A
/// padding leaf is never proven.
pub fn statement_kind(kind: Kind) -> Option<u64> {
    match kind {
        Kind::Kernel => Some(nox_verify::attest::KIND_KERNEL),
        Kind::Capsule => Some(nox_verify::attest::KIND_CAPSULE),
        Kind::Bootloader => Some(nox_verify::attest::KIND_BOOTLOADER),
        Kind::Pad => None,
    }
}

/// Check trailer `t` of `kind` against `root` for context `ctx`, as the gates
/// do: a strict v4 parse for this kind, the path folded to `root`, then the
/// STARK over the statement's words built from `root` and `ctx`. With
/// `allow_path_only`, a development trailer (the v3 path alone) is folded the
/// same way and reported as `Proof::PathOnly`, never as a STARK.
pub fn check_v4(
    root: &[u8; 32],
    kind: Kind,
    ctx: &[u8],
    t: &[u8],
    allow_path_only: bool,
) -> Result<Proof, Refused> {
    if t.starts_with(&MAGIC) {
        if !allow_path_only {
            return Err(Refused::PathOnlyRefused);
        }
        return if verify(root, DEPTH, kind, ctx, t) {
            Ok(Proof::PathOnly)
        } else {
            Err(Refused::Path)
        };
    }
    let v = parse_v4(t, kind, ATTEST.max_proof_bytes).ok_or(Refused::Malformed)?;
    if !verify(root, DEPTH, kind, ctx, v.path) {
        return Err(Refused::Path);
    }
    let words = root_words(root).ok_or(Refused::Root)?;
    let publics = statement_kind(kind)
        .and_then(|k| nox_verify::attest::words(words, ctx, k))
        .ok_or(Refused::Publics)?;
    nox_verify::verify_why(&ATTEST, v.proof, &publics)
        .map(|()| Proof::Stark)
        .map_err(|(r, why)| Refused::Stark(r, why))
}
