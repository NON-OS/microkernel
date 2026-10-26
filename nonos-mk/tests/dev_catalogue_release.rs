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

//! A release market refuses the development catalogue. The catalogue in
//! tools/devnet/market is signed here as a development seal signs it, through
//! this crate's own binary, by a throwaway key made as nonos-dev-image makes
//! one; the market capsule's own ingest, operator trust and ed25519 check are
//! mounted unchanged, its trust built from the operator key this checkout
//! tracks, as a release capsule's is.

#[path = "../../userland/capsule_market/src/bootstrap_trust/mod.rs"]
mod bootstrap_trust;
#[path = "../../userland/capsule_market/src/ingest/mod.rs"]
mod ingest;
#[path = "dev_catalogue/scratch.rs"]
mod scratch;
#[path = "dev_catalogue/signed.rs"]
mod signed;
#[path = "dev_catalogue/verify.rs"]
mod verify;

#[path = "dev_catalogue/release_tests.rs"]
mod release_tests;
