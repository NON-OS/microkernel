// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

mod decode;
mod encode;
mod errno;
mod header;
mod limits;
mod ops;

pub use decode::{parse, refused};
pub use encode::response;
pub use errno::{E_BAD_OP, E_INVAL, E_OK, E_PERM};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::IPC_PAYLOAD_MAX;
pub use ops::{
    OP_BEACON_PARSE, OP_DEVICE_INFO, OP_DMA_STATE, OP_HEALTHCHECK, OP_MGMT_BUILD, OP_RF_STATE,
};
