// NONOS Operating System (AGPL-3.0-or-later)
//! A shutdown's \_TTS(5) or \_PTS(5) cut at every step by the budget (ACPI
//! 6.5, 7.4 and 16.1; recalled, unverified): \_S5 is still read, after
//! whatever the cut method held is given back, so a shutdown cut short
//! still has the sleep types to write.

mod fixture;
mod gen;
mod run;
mod say_tests;
mod sweep_tests;
