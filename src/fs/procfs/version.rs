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

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::sys::clock::civil::from_unix;

/* The release as the VERSION file states it; build.rs passes it in. */
pub const NONOS_VERSION: &str = env!("NONOS_KERNEL_VERSION");
pub const NONOS_CHANNEL: &str = env!("NONOS_RELEASE_CHANNEL");
pub const NONOS_RELEASE: &str =
    concat!(env!("NONOS_KERNEL_VERSION"), "-", env!("NONOS_RELEASE_CHANNEL"));
/* "epoch:<SOURCE_DATE_EPOCH>" when the build pinned its time, else "reproducible:none". */
pub const BUILD_TIME: &str = env!("NONOS_KERNEL_BUILD_TIME");
/* The `rustc --version` line of the compiler that built the kernel. */
pub const COMPILER_VERSION: &str = env!("NONOS_KERNEL_RUSTC_VERSION");

/* The pinned build time in UTC; a build without SOURCE_DATE_EPOCH records none. */
fn build_date() -> String {
    match BUILD_TIME.strip_prefix("epoch:").and_then(|s| s.parse::<u64>().ok()) {
        Some(secs) => {
            let t = from_unix(secs);
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
                t.year, t.month, t.day, t.hour, t.minute, t.second
            )
        }
        None => String::from("build time not recorded"),
    }
}

pub fn read_version() -> String {
    format!("NONOS version {} ({}) ({})\n", NONOS_RELEASE, COMPILER_VERSION, build_date())
}

pub fn get_kernel_version() -> &'static str {
    NONOS_VERSION
}

pub fn get_kernel_release() -> &'static str {
    NONOS_RELEASE
}

pub fn read_version_signature() -> String {
    format!("NONOS {} {} SMP\n", NONOS_RELEASE, build_date())
}

pub fn read_os_release() -> String {
    format!(
        "NAME=\"NONOS\"\nVERSION=\"{v} {c}\"\nID=nonos\nVERSION_ID=\"{v}\"\nPRETTY_NAME=\"NONOS {v} {c}\"\nHOME_URL=\"https://nonos.systems\"\n",
        v = NONOS_VERSION,
        c = NONOS_CHANNEL
    )
}
