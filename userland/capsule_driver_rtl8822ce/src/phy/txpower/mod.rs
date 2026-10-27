// NONOS Operating System (AGPL-3.0-or-later)

//! Transmit power. The 8822C counts power in quarter-dB index steps
//! (`txgi_factor` 2 on rtw88's half-dB efuse diffs; its limit tables read
//! 72 for 18 dBm) and takes it as a reference per path, CCK and OFDM, plus a
//! signed diff per rate (rtw88 `rtw8822c_set_tx_power_index`). Each rate's
//! index is the board's calibrated one from the efuse (`index`), capped at
//! the world domain's lowest 2.4 GHz EIRP limit less a 2 dBi antenna gain,
//! the RTL8821CE's assumptions, until the power-by-rate and limit tables are
//! ported. It is written once, the lowest over every 2.4 GHz channel group,
//! so it fits each channel a scan or a join tunes to.

pub mod index;
pub mod write;

pub use write::set_tx_power;
