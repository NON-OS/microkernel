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

//! Device enrollment, the TPM's half: what a registrar needs to bind a
//! device's commitment to a genuine TPM, and nothing more.
//!
//! - [`ek_public`]: the endorsement key, derived from the TCG EK Credential
//!   Profile's default template, so it is the key the EK certificate certifies.
//! - [`ek_certificate`]: that certificate, out of its NV index.
//! - [`ak_public`]: the attestation key's public area and name.
//! - [`activate_credential`]: the answer to the registrar's MakeCredential,
//!   which the TPM gives only while the EK and the AK it names are both its own.
//! - [`ak_sign`]: the AK's signature over a 32-byte message, checked by the
//!   registrar against `ak_public`.
//!
//! Every command goes through `machine_key::run`. Each step is a pure builder
//! and a pure parser, so the bytes can be checked without a TPM.

pub(in crate::security::tpm) mod activate;
pub(in crate::security::tpm) mod activate_call;
pub(in crate::security::tpm) mod activated;
pub(in crate::security::tpm) mod ak_calls;
pub(in crate::security::tpm) mod ak_public;
pub(in crate::security::tpm) mod cert;
pub(in crate::security::tpm) mod consts;
pub(in crate::security::tpm) mod ek;
pub(in crate::security::tpm) mod ek_calls;
pub(in crate::security::tpm) mod error;
pub(in crate::security::tpm) mod hash;
pub(in crate::security::tpm) mod kind;
pub(in crate::security::tpm) mod marshal;
pub(in crate::security::tpm) mod nv_public;
pub(in crate::security::tpm) mod nv_read;
pub(in crate::security::tpm) mod policy;
pub(in crate::security::tpm) mod public;
pub(in crate::security::tpm) mod sign;
pub(in crate::security::tpm) mod template;

pub use activate_call::activate_credential;
pub use activated::Activated;
pub use ak_calls::{ak_public, ak_sign};
pub use cert::ek_certificate;
pub use consts::{AK_SIGN_LABEL, CERT_MAX, ENCRYPTED_SECRET_MAX, ID_OBJECT_MAX, NAME_LEN};
pub use ek_calls::ek_public;
pub use error::EnrollError;
pub use kind::EkKind;
pub use public::Public;
