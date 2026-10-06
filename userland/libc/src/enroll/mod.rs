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

//! Registering this device: the TPM's half, through `MkEnroll`, for the
//! capsule that holds DeviceSecret.

mod call;
mod frame;

pub use call::{activate, ak_public, ak_sign, ek_certificate, ek_public, mk_enroll};
pub use frame::{
    frame_challenge, split_public, AK_SIGN_LABEL, CERT_MAX, CHALLENGE_MAX, EK_ECC_P256, EK_RSA2048,
    NAME_LEN, PUBLIC_MAX, SECRET_MAX,
};
