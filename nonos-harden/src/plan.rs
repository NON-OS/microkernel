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

//! The checks, in the order they run. Every one runs every time: there is no
//! way to skip one, and the proofs fail if one goes missing from this list.
//! A check whose input an earlier one did not establish says it did not run.

use crate::check::Check;
use crate::cli::CheckArgs;
use crate::ctx::Ctx;
use crate::deliver::deliver;
use crate::exit;
use crate::release::{kernel_check, kind, load, receipt_check, sealed};
use crate::sign::{self, key};
use crate::tree::runner::{Runner, System};
use crate::tree::{find, inputs, ledger, matrix, source, verify};
use crate::verdict::{self, text::clean};

pub type Step = (&'static str, fn(&mut Ctx) -> Check);

pub const STEPS: &[Step] = &[
    (load::ID, load::check),
    (sealed::ID, sealed::check),
    (kind::ID, kind::check),
    (receipt_check::ID, receipt_check::check),
    (source::ID, source::check),
    (ledger::ID, ledger::check),
    (kernel_check::ID, kernel_check::check),
    (inputs::ID, inputs::check),
    (verify::HYGIENE, verify::hygiene),
    (verify::TRUST_CHAIN, verify::trust_chain),
    (verify::SUPPLY_CHAIN, verify::supply_chain),
    (matrix::STATIC, matrix::static_checks),
    (matrix::PROOFS, matrix::proof_crates),
];

pub fn run(ctx: &mut Ctx) -> Vec<Check> {
    STEPS.iter().map(|(_, step)| step(ctx)).collect()
}

pub fn check(args: &CheckArgs) -> u8 {
    check_with(args, Box::new(System))
}

/// The key is read and vetted before any check runs: a key that will be
/// refused should not cost the hours the checks can take.
pub fn check_with(args: &CheckArgs, runner: Box<dyn Runner>) -> u8 {
    let here = || std::env::current_dir().ok().and_then(|d| find(&d));
    let tree = args.tree.clone().or_else(here);
    let key = match args.sign_key.as_deref().map(|k| key::load(k, tree.as_deref())) {
        None => None,
        Some(Ok(k)) => Some(k),
        Some(Err(e)) => {
            eprintln!("nonos-harden: not signing with that key: {}", clean(&e));
            return exit::USAGE;
        }
    };
    let mut ctx = Ctx::with_runner(args.release.clone(), tree, runner);
    let checks = run(&mut ctx);
    let v = verdict::build(&ctx, checks, key.as_ref().map(sign::signer));
    deliver(&v, args, key.as_ref())
}
