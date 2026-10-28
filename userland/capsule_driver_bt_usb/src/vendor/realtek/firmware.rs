// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The RTL8821C Bluetooth patch and its config, linux-firmware's files,
//! embedded unmodified (firmware/README.md says why they may be, and why
//! they are not a package as MediaTek's are). The capsule's signature
//! covers them. Their pins are firmware/SHA256SUMS, which
//! nonos_bt_rtl_proofs holds these very bytes to, and
//! scripts/baselines/upstream.sha256, which scripts/check_prebuilt.py
//! holds the tree's files to.

pub const FW: &[u8] = include_bytes!("../../../firmware/rtl8821c_fw.bin");
pub const CONFIG: &[u8] = include_bytes!("../../../firmware/rtl8821c_config.bin");
