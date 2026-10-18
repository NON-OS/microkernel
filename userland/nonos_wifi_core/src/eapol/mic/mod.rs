// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

mod mickind;
mod verify_mic_kind;
use super::parse::{HEADER_LEN, MIC_LEN, MIC_OFFSET};
use crate::ccmp::cmac::aes_cmac_parts;
use crate::wpa::hmac::hmac_sha1_parts;
pub use mickind::*;
pub use verify_mic_kind::*;
