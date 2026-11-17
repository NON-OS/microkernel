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

//! About's `data` on the host: the plain tables mounted as they ship, and a
//! fixed answer in place of each kernel read, so every screen paints the same
//! content at every scale.

#[path = "../../capsule_about/src/about/data/abi.rs"]
pub mod abi;
#[path = "../../capsule_about/src/about/data/admission.rs"]
pub mod admission;
#[path = "../../capsule_about/src/about/data/caps.rs"]
pub mod caps;
#[path = "about_double_doc.rs"]
mod doc;
#[path = "../../capsule_about/src/about/data/layout.rs"]
pub mod layout;
#[path = "../../capsule_about/src/about/data/license.rs"]
pub mod license;
#[path = "../../capsule_about/src/about/data/own_caps.rs"]
pub mod own_caps;
#[path = "../../capsule_about/src/about/data/present.rs"]
pub mod present;
#[path = "../../capsule_about/src/about/data/product.rs"]
pub mod product;
#[path = "about_double_proofs.rs"]
pub mod proofs;
#[path = "about_double_reads.rs"]
mod reads;
#[path = "../../capsule_about/src/about/data/third_party.rs"]
pub mod third_party;
#[path = "../../capsule_about/src/about/data/trust.rs"]
pub mod trust;
#[path = "about_double_verify.rs"]
pub mod verify;

pub use doc::{attest_doc, doc_parse};
pub use reads::{build, display, own, runtime, uptime};
