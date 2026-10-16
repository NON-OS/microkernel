// NONOS Operating System (AGPL-3.0-or-later)
//! The gen3 memory plan, both queues, and every command payload's byte image.
//!
//! The structure sizes and field offsets pinned here were compiled from the
//! Linux v6.12 iwlwifi headers (fw/api/*.h, iwl-context-info*.h, iwl-fh.h,
//! pcie/internal.h) with gcc and printed with `sizeof`/`offsetof`, so a
//! transcription slip in a builder cannot pass: iwl_tfh_tfd 256,
//! iwl_rx_transfer_desc 16, iwl_rx_completion_desc 32 (rbid at 4, flags at
//! 6), iwl_cmd_header_wide 8, iwl_mac_config_cmd 52 (action 4, mac_type 8,
//! local_mld_addr 12, filter_flags 20, nic_not_ack_enabled 32),
//! iwl_link_config_cmd 208 (link_id 4, mac_id 8, phy_id 12, local_link_addr
//! 16), iwl_scan_config 12 (tx_chains 4, rx_chains 8), iwl_mcc_update_cmd 28,
//! iwl_scan_req_umac_v17 1940 (general params at 8, channel params at 44,
//! periodic at 584, probe at 596; in the general params: link id 3, active
//! dwell 4, adaptive dwell defaults 6..8, max budget 10, priority 28, passive
//! dwell 32), iwl_scan_channel_cfg_umac 8.

mod clear;
mod cmdq;
mod commands;
mod image;
mod regions;
mod rxq;
mod scan;
