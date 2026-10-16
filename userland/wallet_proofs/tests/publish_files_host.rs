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

//! The publisher's files at their caps: the entry document and the attest log
//! taken whole at the cap, one byte more refused.
//
// publish/files.rs is the real wallet source, mounted at crate::publish so
// its `super::super::attest` is this crate's attest: the lib's own mount of
// the real attest log. It names `nonos_app_skeleton`, which this target
// answers as itself: the vfs client is a host stand-in (clients.rs).

extern crate alloc;
extern crate self as nonos_app_skeleton;

#[path = "publish_files_host/clients.rs"]
pub mod clients;
#[path = "publish_files_host/doc_tests.rs"]
mod doc_tests;
#[path = "publish_files_host/document.rs"]
mod document;
#[path = "publish_files_host/entry.rs"]
mod entry;
#[path = "publish_files_host/log_tests.rs"]
mod log_tests;
#[path = "publish_files_host/publish.rs"]
mod publish;
#[path = "publish_files_host/release.rs"]
mod release;

pub use wallet_proofs::attest;
