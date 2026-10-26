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

//! A development image takes the development catalogue its seal signed, and
//! the catalogue's schema 3 bytes keep every commerce block. The image's
//! market is the capsule's own ingest and ed25519 check with its operator
//! trust built from a throwaway key, as the dev tree's capsule is built from
//! the key nonos-dev-image made there.

#[path = "dev_catalogue/dev_trust.rs"]
mod bootstrap_trust;
#[path = "../../userland/capsule_market/src/ingest/mod.rs"]
mod ingest;
#[path = "dev_catalogue/scratch.rs"]
mod scratch;
#[path = "dev_catalogue/signed.rs"]
mod signed;
#[path = "dev_catalogue/verify.rs"]
mod verify;

#[path = "dev_catalogue/codec_tests.rs"]
mod codec_tests;
#[path = "dev_catalogue/image_tests.rs"]
mod image_tests;
