//! Joining a remembered Wi-Fi network at boot, once the Wi-Fi driver is up.

mod attempt;
mod join;
mod link;
mod machine;
mod ready;
mod relink;
mod saved;
mod tick;

pub use tick::tick;
