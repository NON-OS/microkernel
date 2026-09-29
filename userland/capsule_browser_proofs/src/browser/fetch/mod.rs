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

//! The fetch machine's pure half, from the capsule's own files.

#[path = "../../../../capsule_browser/src/browser/fetch/budget.rs"]
pub mod budget;
#[path = "../../../../capsule_browser/src/browser/fetch/connect.rs"]
pub mod connect;
#[path = "../../../../capsule_browser/src/browser/fetch/constants.rs"]
pub mod constants;
#[path = "../../../../capsule_browser/src/browser/fetch/deadline.rs"]
pub mod deadline;
#[path = "../../../../capsule_browser/src/browser/fetch/expire.rs"]
pub mod expire;
#[path = "../../../../capsule_browser/src/browser/fetch/keep.rs"]
pub mod keep;
#[path = "../../../../capsule_browser/src/browser/fetch/open.rs"]
pub mod open;
#[path = "../../../../capsule_browser/src/browser/fetch/plain/mod.rs"]
pub mod plain;
#[path = "../../../../capsule_browser/src/browser/fetch/pool/mod.rs"]
pub mod pool;
#[path = "../../../../capsule_browser/src/browser/fetch/progress.rs"]
pub mod progress;
#[path = "../../../../capsule_browser/src/browser/fetch/retryable_error.rs"]
pub mod retryable_error;
#[path = "../../../../capsule_browser/src/browser/fetch/reuse.rs"]
pub mod reuse;
#[path = "../../../../capsule_browser/src/browser/fetch/run.rs"]
pub mod run;
#[path = "../../../../capsule_browser/src/browser/fetch/security_error.rs"]
pub mod security_error;
#[path = "../../../../capsule_browser/src/browser/fetch/socks/mod.rs"]
pub mod socks;
#[path = "../../../../capsule_browser/src/browser/fetch/tls/mod.rs"]
pub mod tls;
#[path = "../../../../capsule_browser/src/browser/fetch/tls_reason.rs"]
pub mod tls_reason;
#[path = "../../../../capsule_browser/src/browser/fetch/types/mod.rs"]
pub mod types;
#[path = "../../../../capsule_browser/src/browser/fetch/wire.rs"]
pub mod wire;
