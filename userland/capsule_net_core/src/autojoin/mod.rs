//! Joining a remembered Wi-Fi network at boot, once the Wi-Fi driver is up.

mod attempt;
mod machine;
mod ready;
mod tick;

pub use tick::tick;
