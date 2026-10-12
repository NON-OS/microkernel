// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's waits on a command, run against a model port. The device is
//! the adversary here: it may post any status, and it may finish between any
//! two register reads the driver makes.

mod honest;
mod hostile;
mod hostile_fixed;
mod hostile_port;
mod hostile_random;
mod hostile_ready;
mod model_port;
mod stale_status;
