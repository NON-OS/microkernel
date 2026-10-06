/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

use super::default_hostname::default_hostname;
use super::empty_string::empty_string;
use crate::store::types::Store;

pub const fn store() -> Store {
    Store {
        brightness: 80,
        mouse_sensitivity: 2,
        sound_enabled: true,
        anonymous_mode: true,
        nym_enabled: false,
        theme: 0,
        keyboard_layout: 0,
        auto_wipe: true,
        timezone: 0,
        screen_timeout: 0,
        language: 0,
        developer_mode: false,
        hardware_crypto: true,
        zk_attestation: true,
        system_keys_generated: false,
        notifications_enabled: true,
        high_contrast: false,
        font_size: 1,
        auto_lock_timeout: 5,
        wifi_autoconnect: true,
        animations_enabled: true,
        cursor_size: 1,
        // special-variant-9: catalog index 13 + 14 + 18 + 10.
        wallpaper: 55,
        clock_format24: true,
        prefer_ipv6: false,
        metered_connection: false,
        proxy_mode: 0,
        wifi_radio: true,
        wifi_ask_to_join: true,
        volume: 64,
        audio_balance: 50,
        alert_sounds: false,
        startup_chime: false,
        persistent: false,
        apps_off: 0,
        wallpapers_kept: nonos_policy_proto::wallpapers_kept::ALL,
        // The Nym mixnet until setup or Settings says otherwise.
        network_route: nonos_policy_proto::route::NYM,
        kernel_aslr: true,
        kernel_stack_guard: true,
        kernel_nx_bit: true,
        kernel_smep: true,
        kernel_smap: true,
        kernel_debug: false,
        kernel_serial: true,
        kernel_watchdog: false,
        kernel_preempt: true,
        kernel_hugepages: false,
        kernel_iommu: true,
        kernel_seccomp: true,
        hostname: default_hostname(),
        domainname: empty_string(),
        username: empty_string(),
        qwen_tier: empty_string(),
    }
}
