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

//! The tests, one file per step of the check, and the helpers they share.

use nsh_attest::Refusal;

use crate::build::Spec;

mod challenge;
mod dma;
mod doc_fields;
mod doc_lengths;
mod doc_version;
mod enrol;
mod enrol_boot;
mod enrol_halves;
mod enrol_sig;
mod firmware;
mod front;
mod happy;
mod host_proof;
mod kernel_encoder;
mod log_caps;
mod log_diff;
mod log_doc;
mod log_event;
mod log_header;
mod log_header_cut;
mod log_ignored;
mod log_loader;
mod log_locality;
mod log_pcr4;
mod log_replay;
mod names;
mod numbers;
mod p256_kat;
mod policy;
mod policy_releases;
mod quote_binding;
mod quote_digest;
mod quote_select;
mod quote_sig;
mod quote_sig_range;
mod quote_sig_shape;
mod quote_struct;
mod quote_v3;
mod quote_v3_order;
mod registry;
mod release;
mod robust;

/// The refusal a spec's host meets.
pub fn refused(s: Spec) -> Refusal {
    s.build().check().expect_err("this host must be refused")
}

/// A spec's host passes.
pub fn passes(s: Spec) {
    s.build().check().expect("this host must pass");
}
