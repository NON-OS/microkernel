// NONOS Operating System (AGPL-3.0-or-later)
//! The tests of a medium that may have changed under the device served:
//! what a sense, or a command lost on the bus, says of it, what the check
//! makes of that, what holds it for the kernel, and how long a device that
//! keeps failing is kept. Each in its own file beside this one.

#[path = "afresh_tests.rs"]
mod afresh_tests;
#[path = "attention_tests.rs"]
mod attention_tests;
#[path = "card_out_tests.rs"]
mod card_out_tests;
#[path = "csw_tests.rs"]
mod csw_tests;
#[path = "fresh_tests.rs"]
mod fresh_tests;
#[path = "hold_tests.rs"]
mod hold_tests;
#[path = "lost_tests.rs"]
mod lost_tests;
#[path = "refusal_tests.rs"]
mod refusal_tests;
#[path = "row_count_tests.rs"]
mod row_count_tests;
#[path = "row_tests.rs"]
mod row_tests;
#[path = "start_unit_tests.rs"]
mod start_unit_tests;
