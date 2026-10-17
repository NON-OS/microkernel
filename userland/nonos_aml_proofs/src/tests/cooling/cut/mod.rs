// NONOS Operating System (AGPL-3.0-or-later)
//! F-127: a thermal zone whose _TMP, _CRT, _HOT and _PSV are cut by the
//! budget at every step, or answer lies: below absolute zero, past any
//! plausible bound, not an integer, or out of order. The model, stated
//! apart from the code:
//! - a trip object whose method finished with an integer in the believed
//!   range (-50 to 200 °C, the F ruling) is believed as read;
//! - one cut short, not an integer, or outside that range is unread: the
//!   value believed before is kept and its unread bit set, so a trip is
//!   never dropped for it; an absent trip is None;
//! - _AC0 and _AC1 the same, kept at their level, falling from _AC0; one
//!   absent, warmer, or unread with none kept ends the list (F-127b);
//! - _TZP is any integer, else the period before is kept;
//! - _HOT is then judged by the F-110b order rule (`trip_rule`, proved in
//!   `trip_rule_tests`), so a lying order fails toward the safer trip;
//! - _TMP the same: a reading or nothing, never a wrong number;
//! - a zone's watch (`watch_model`): an unread reading changes nothing
//!   believed, and _CRT holds only on two real readings at or past it a
//!   second or more apart, with none below it between;
//! - nothing is held after any read.

mod fixture;
mod reader;
mod tmp_tests;
mod trips_model;
mod trips_tests;
mod watch_model;
mod watch_tests;
