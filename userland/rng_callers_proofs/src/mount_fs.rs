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

// The kernel's src/fs, included into the crate root: the sealed sector,
// ramfs, cryptofs whole, and the data volume's keying. The RAM disk and
// the block filesystem stand in. The volume's proofs sit inside its module,
// where its `pub(super)` calls are in reach.
#[allow(clippy::all)]
#[path = "../../../src/fs"]
pub mod fs {
    pub use crate::stand_in::blockfs;
    pub mod cryptofs;

    pub mod cryptoblock {
        mod constants;
        mod epoch;
        mod error;
        mod open;
        mod seal;
        mod sector_open;
        mod window;
        pub use crate::stand_in::ram;
        pub use constants::{PLAIN_BLOCK_BYTES, SECTOR_BYTES};
        pub use error::CryptoBlockError;
        pub use open::{open, open_into};
        pub use seal::seal;
        pub use window::{set_window, window_sectors};
    }

    pub mod ramfs {
        pub mod error;
        pub mod types;
        pub mod filesystem {
            pub mod core;
            pub mod crypto;
            pub mod key;
            pub mod ops;
            pub mod ops_write;
            pub mod path;
            pub use self::core::NonosFilesystem;
        }
    }

    pub mod blockfs_volume {
        pub mod error;
        pub mod format_volume;
        mod key_header;
        mod key_header_io;
        mod key_seal;
        mod mount_or_format;
        mod opened;
        mod passphrase_create;
        mod passphrase_key;
        pub mod plan_types;
        mod ring_blank;
        mod say;
        mod session;
        mod state;
        #[path = "../../../userland/rng_callers_proofs/src/volume_tests/mod.rs"]
        mod tests;
    }
}
