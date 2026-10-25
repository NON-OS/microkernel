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

//! The contracts the OS calls, reads or signs for. NOX is the kept token; the
//! rest are the owner's new marketplace set, the Shield pool a
//! meter is funded from, and that pool's AmountPolicy (v1.4.17).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contract {
    NoxToken,
    Settlement,
    Escrow,
    PublisherBond,
    ListingRegistry,
    ProofAnchor,
    ShieldPool,
    AmountPolicy,
}

/// Every contract, in the order settle checks them at start.
pub const CONTRACTS: [Contract; 8] = [
    Contract::NoxToken,
    Contract::Settlement,
    Contract::Escrow,
    Contract::PublisherBond,
    Contract::ListingRegistry,
    Contract::ProofAnchor,
    Contract::ShieldPool,
    Contract::AmountPolicy,
];
