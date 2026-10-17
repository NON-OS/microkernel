// NONOS Operating System (AGPL-3.0-or-later)
//! F-125: one SCI's GPE method cut at every step by the budget, and the
//! drain of what it notified, its deliveries cut too. The model, from the
//! event layer's rule (ACPI 6.5, 5.6.4.1; recalled, unverified): the GPE
//! is disabled while served; an edge GPE's status is cleared before its
//! method and a level one's after; a GPE whose method the budget cut is
//! enabled again, until `CUTS` runs in a row were cut. Nothing is held
//! after, and each notification the method queued before the cut is
//! delivered once, in order.

mod cuts_tests;
mod fixture;
mod run;
mod sweep_tests;
