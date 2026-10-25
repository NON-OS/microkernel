# Hardware support matrix: Ethernet

Part of [Hardware support matrix](MATRIX.md).

## Ethernet

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Ethernet | Intel e1000, 28 ids, 8086:100e among them | `capsule_driver_e1000` | Partial: `net.core` drops every frame received through this driver, so DHCP gets no lease ([receive fault](../drivers/ethernet/README.md#the-receive-fault)) | proof crate `e1000_proofs` (20) | 0.9.2 |
| Ethernet | Intel e1000e: 82574, 82583, I217, I218, I219, 64 ids | `capsule_driver_e1000e` | Not supported: not in the image | proof crate `e1000e_proofs` (48) | 0.9.2 |
| Ethernet | Realtek RTL8139 10ec:8139 | `capsule_driver_rtl8139` | Partial: the same [receive fault](../drivers/ethernet/README.md#the-receive-fault) as e1000 | proof crate `rtl8139_proofs` (15) | 0.9.2 |
| Ethernet | Realtek RTL8169, RTL8168 and RTL8111, RTL810x, RTL8125: 10ec:8169, 8167, 8161, 8162, 8168, 2502, 2600, 8136, 8125, 3000 | `capsule_driver_rtl8169` | Partial: the same [receive fault](../drivers/ethernet/README.md#the-receive-fault) as e1000 | proof crate `rtl8169_proofs`: its 67 tests pass, then the check fails at this commit on a clippy lint | 0.9.2 |
| Ethernet | Realtek RTL8126 10ec:8126, RTL8127 10ec:8127 | none | Not supported: left out of the RTL8169 driver, since their start differs from the RTL8125's | none | 0.9.2 |
| Ethernet | virtio-net 1af4:1000, 1af4:1041 | `capsule_driver_virtio_net` | Works | proof crate `virtio_net_proofs` (20); QEMU `virtio-net-pci` | 0.9.2 |
| Ethernet | USB CDC-ECM (class 02/06), CDC-NCM (02/0d), RNDIS (02/02/ff, e0/01/03, ef/04/01) | `capsule_driver_cdc_ecm`, `capsule_driver_cdc_ncm`, `capsule_driver_rndis` | Not supported: not in the image | proof crates `cdc_ecm_proofs` (7), `cdc_ncm_proofs` (33), `rndis_proofs` (22) | 0.9.2 |
| Ethernet | ASIX AX88179 and AX88178A, USB 0b95:1790, 0b95:178a and 11 more | `capsule_driver_ax88179` | Not supported: not in the image | proof crate `ax88179_proofs` (25) | 0.9.2 |
| Ethernet | Realtek RTL8153, USB 0bda:8153 and 18 more | `capsule_driver_rtl8153` | Not supported: not in the image | proof crate `rtl8153_proofs` (27) | 0.9.2 |
