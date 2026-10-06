// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The four-way and group-key handshake message layer: parsing EAPOL-Key
//! frames, verifying their MIC under the KCK derived from the pairwise
//! transient key, and reading the key data they carry. The MIC rests on the
//! RFC-verified HMAC-SHA1 (WPA2-PSK) and AES-CMAC (SAE, PSK-SHA256).

pub mod build;
pub mod kde;
pub mod mic;
pub mod parse;
