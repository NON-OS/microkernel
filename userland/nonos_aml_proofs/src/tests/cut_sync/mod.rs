// NONOS Operating System (AGPL-3.0-or-later)
//! Mutex Acquire and Release, \_GL, Notify, Sleep and Stall cut at every
//! step (ACPI 6.5, 19.6.2, 19.6.117, 19.6.95, 19.6.128, 19.6.129 and
//! 5.2.10.1; recalled, unverified): whatever the evaluation held when it
//! was cut is given back, and what it did before the cut is what the uncut
//! run did up to there.

mod fixture;
mod gen;
mod held;
mod model;
mod run;
mod sweep_tests;
