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

//! The shield's region set as a closed enum. The wired engine holds regions
//! as boxed trait objects, which is right for layout and proving but cannot
//! carry a generic method — and the recursive verifier needs every region's
//! transition over the tower, not the base field. Naming the four concrete
//! types keeps that dispatch typed: no downcasting, and a region type outside
//! this list cannot silently lose its in-circuit recomputation.

use super::index_scalar::IndexScalar;
use super::multi_membership::MultiMembership;
use super::publics::Publics;
use super::spec::AirExt;
use super::value_balance::ValueBalance;
use crate::field::Felt;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Every region kind the shield stacks. Balance, ten memberships (notes,
/// pool, keys, association), two index scalars, and the publics row.
#[derive(Clone)]
pub enum ShieldRegion {
    Balance(ValueBalance),
    Membership(MultiMembership),
    Index(IndexScalar),
    Publics(Publics),
}

impl ShieldRegion {
    /// The region as the trait object the wired engine stacks.
    pub fn boxed(&self) -> Box<dyn AirExt> {
        match self {
            ShieldRegion::Balance(r) => Box::new(r.clone()),
            ShieldRegion::Membership(r) => Box::new(r.clone()),
            ShieldRegion::Index(r) => Box::new(r.clone()),
            ShieldRegion::Publics(r) => Box::new(r.clone()),
        }
    }

    /// The region seen through the trait, for layout queries before boxing.
    pub fn as_air(&self) -> &dyn AirExt {
        match self {
            ShieldRegion::Balance(r) => r,
            ShieldRegion::Membership(r) => r,
            ShieldRegion::Index(r) => r,
            ShieldRegion::Publics(r) => r,
        }
    }

    /// The region's transition over any field, for the recursive verifier.
    pub fn transition_gen<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        match self {
            ShieldRegion::Balance(r) => r.transition_gen(window, periodic),
            ShieldRegion::Membership(r) => r.transition_gen(window, periodic),
            ShieldRegion::Index(r) => r.transition_gen(window, periodic),
            ShieldRegion::Publics(r) => r.transition_gen(window, periodic),
        }
    }
}
