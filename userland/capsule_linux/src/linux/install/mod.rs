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

//! Installing a Linux program from within the system.

mod apps;
mod apps_coder;
mod apps_qwen25;
mod apps_qwen3;
mod apps_qwen35;
mod apps_run;
/* Waiting for Anyone before an install downloads, the fetcher's own rule. */
#[path = "../../../../capsule_model_fetch/src/get/anyone_wait.rs"]
mod anyone_wait;
mod auth;
mod chat_build;
mod chat_pick;
mod deb;
mod exports;
mod family;
mod fetch;
/* The model fetcher's exit statuses, from its own source: one table. */
#[path = "../../../../capsule_model_fetch/src/exit.rs"]
mod fetch_exit;
mod get;
mod hex;
mod http;
mod http_reply;
mod index;
mod index_find;
mod isa;
mod limit;
mod manifest;
mod market;
mod member;
mod mirror;
mod model_dep;
mod model_remove;
mod package_record;
mod pacman;
mod pgp;
mod pkg;
mod place;
mod place_links;
mod proof_names;
mod record;
mod refuse;
mod remove;
mod route_read;
mod run;
mod tar;
mod tar_field;
mod tar_kind;
mod tar_path;
mod tar_pax;
mod tcg;
mod tier;
mod tools;
mod tools_install;
mod unpacked;
mod why;

pub use exports::*;
