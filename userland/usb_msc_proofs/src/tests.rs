// NONOS Operating System (AGPL-3.0-or-later)
//! The tests, each in its own file beside this one; those of a medium
//! that may have changed are in change_tests.rs.

#[path = "admits_tests.rs"]
mod admits_tests;
#[path = "backoff_tests.rs"]
mod backoff_tests;
#[path = "bound_tests.rs"]
mod bound_tests;
#[path = "check_tests.rs"]
mod check_tests;
#[path = "codec_tests.rs"]
mod codec_tests;
#[path = "controller_tests.rs"]
mod controller_tests;
#[path = "device_time_tests.rs"]
mod device_time_tests;
#[path = "empty_reader_tests.rs"]
mod empty_reader_tests;
#[path = "generation_tests.rs"]
mod generation_tests;
#[path = "identity_tests.rs"]
mod identity_tests;
#[path = "inquiry_tests.rs"]
mod inquiry_tests;
#[path = "lun_tests.rs"]
mod lun_tests;
#[path = "medium_tests.rs"]
mod medium_tests;
#[path = "msc_tests.rs"]
mod msc_tests;
#[path = "reask_empty_tests.rs"]
mod reask_empty_tests;
#[path = "reask_tests.rs"]
mod reask_tests;
#[path = "rebind_tests.rs"]
mod rebind_tests;
#[path = "reply_fuzz_tests.rs"]
mod reply_fuzz_tests;
#[path = "reply_tests.rs"]
mod reply_tests;
#[path = "request_bound_tests.rs"]
mod request_bound_tests;
#[path = "request_refusal_tests.rs"]
mod request_refusal_tests;
#[path = "residue_found_tests.rs"]
mod residue_found_tests;
#[path = "residue_tests.rs"]
mod residue_tests;
#[path = "sense_lost_tests.rs"]
mod sense_lost_tests;
#[path = "serial_tests.rs"]
mod serial_tests;
#[path = "slot_call_tests.rs"]
mod slot_call_tests;
#[path = "span_tests.rs"]
mod span_tests;
#[path = "transport_tests.rs"]
mod transport_tests;
#[path = "window_tests.rs"]
mod window_tests;
