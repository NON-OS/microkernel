/*
 * NONOS Operating System (AGPL-3.0-or-later)
 */

//! The saved-network modules of `nonos_wifi_client`, grouped so their
//! `super::` references resolve as they do in the crate.

#[path = "../../../nonos_wifi_client/src/saved/error.rs"]
mod error;
#[path = "../../../nonos_wifi_client/src/saved/file.rs"]
mod file;
#[path = "../../../nonos_wifi_client/src/saved/list.rs"]
mod list;
#[path = "../../../nonos_wifi_client/src/saved/list_codec.rs"]
mod list_codec;
mod tests;
