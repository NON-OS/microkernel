// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The firmware file header and the API versions accepted.

pub const IWL_FW_MAGIC: u32 = 0x0A4C_5749;
pub const FW_API_VERSION_MASK: u32 = 0xFFFF;
pub const MIN_FW_API_VERSION: u16 = 22;
/// The newest image bundled (so-a0-gf-a0-86); 77 refused it outright.
pub const MAX_FW_API_VERSION: u16 = 86;
