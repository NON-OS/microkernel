// NONOS Operating System (AGPL-3.0-or-later)
//! F-109: AML synchronization against hostile firmware. Seeded methods of
//! Acquire, Release, Wait, Signal and Reset over mutexes of every sync
//! level, \_GL, events, and mutexes another evaluation left held, under
//! budgets small and large, each held to a model stated apart.

mod gen;
mod model;
mod sweep_tests;
mod table;
