/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The capsule's MediaTek setup, every file but its glue, unchanged,
//! against a modelled MT7921/MT7922 (`model`) and synthetic patch files
//! built here (`build`): never MediaTek's bytes.

#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/chip.rs"]
pub mod chip;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/fail.rs"]
pub mod fail;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/fetch.rs"]
pub mod fetch;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/files.rs"]
pub mod files;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/ids.rs"]
pub mod ids;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/image.rs"]
pub mod image;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/load.rs"]
pub mod load;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/pins.rs"]
pub mod pins;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/port.rs"]
pub mod port;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/reg.rs"]
pub mod reg;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/reset.rs"]
pub mod reset;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/setup.rs"]
pub mod setup;
#[path = "../../../../capsule_driver_bt_usb/src/vendor/mediatek/wmt.rs"]
pub mod wmt;

mod build;
mod model;
mod model_port;
mod model_wmt;

mod applied_tests;
mod busy_tests;
mod chip_tests;
mod image_tests;
mod phase_tests;
mod pin_tests;
