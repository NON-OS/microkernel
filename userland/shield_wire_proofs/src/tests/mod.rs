// NONOS Operating System (AGPL-3.0-or-later)
//! The service builds each reply with its own builders, wraps a job's in
//! the RESULT envelope, frames it; the wallet unframes it and reads it with
//! its own readers. A mocked pool and lander drive the follow minute by
//! minute, and every minute the wallet must read what the service said.

mod common;
mod history;
mod kept;
mod landed;
mod owner;
mod two_in_flight;
mod values;
