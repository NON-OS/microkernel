// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

pub mod blob;
pub mod family;
pub mod carried;
pub mod gen3;
pub mod generation;

pub use blob::{gen3_blob, gen3_pnvm};
pub use family::{family_for_device, Family};
