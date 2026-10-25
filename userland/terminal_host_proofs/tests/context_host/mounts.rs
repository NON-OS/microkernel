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

//! The context, cwd and identity modules, mounted by path.

#[path = "../../../capsule_terminal/src/term"]
pub mod term {
    pub mod cwd {
        mod home;
        mod shorten;
        mod strip_home;
        pub use home::HOME;
        pub use shorten::shorten;
        pub use strip_home::strip_home;
    }

    pub mod util {
        mod copy_into;
        pub use copy_into::copy_into;
    }

    pub mod identity {
        mod choose;
        mod sanitize;
        mod wire;
        pub use choose::{choose, HOST_FALLBACK, USER_FALLBACK};
        pub use sanitize::{hostname_len, user_len};
        pub use wire::{decode_str, request, REQ_LEN};
    }

    pub mod context;
    pub mod prompt;
}
