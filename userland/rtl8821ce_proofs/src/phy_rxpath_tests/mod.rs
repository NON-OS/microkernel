// NONOS Operating System (AGPL-3.0-or-later)

//! Proofs for the receiver-enable path (`phy::rxpath`): the RX-path reset toggle
//! around the table load, the RFE antenna switch that routes the antenna into the
//! WiFi low-noise amplifier, the 2.4GHz receive-path select, and the receive
//! digital filter. A modeled device stores register values so read-modify-writes
//! behave, and records the last value written to each offset.

mod antenna;
mod model;
mod power;
