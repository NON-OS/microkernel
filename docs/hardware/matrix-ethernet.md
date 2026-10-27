# Hardware support matrix: Ethernet

Part of [Hardware support matrix](MATRIX.md).

## Ethernet

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Ethernet | Intel e1000, 28 ids, 8086:100e among them | `capsule_driver_e1000` | Works on QEMU: a DHCP lease through the card, read in batches; the receive fault is fixed | proof crate `e1000_proofs` (47); QEMU lane `nic=e1000`, [evidence](../release-evidence/0.9.3/drivers/e1000/) | 0.9.3 |
| Ethernet | Realtek RTL8139 10ec:8139 | `capsule_driver_rtl8139` | Works on QEMU: DHCP lease, batches; the receive fault is fixed | proof crate `rtl8139_proofs` (48); QEMU lane `nic=rtl8139`, [evidence](../release-evidence/0.9.3/drivers/rtl8139/) | 0.9.3 |
| Ethernet | Realtek RTL8169, RTL8168 and RTL8111, RTL810x, RTL8125: 10ec:8169, 8167, 8161, 8162, 8168, 2502, 2600, 8136, 8125, 3000 | `capsule_driver_rtl8169` | Integrated, host-proven, not booted on hardware: the receive-fault fix and the DMA grant checks are proven on the host; QEMU has no model ([desktop board plan](bring-up/desktop-board.md)) | proof crate `rtl8169_proofs` (92), clippy clean | 0.9.3 |
| Ethernet | AMD PCnet 1022:2000 (virtual machines) | `capsule_driver_pcnet` | Works on QEMU: DHCP lease, batches | proof crate `pcnet_proofs` (27); QEMU lane `nic=pcnet`, [evidence](../release-evidence/0.9.3/drivers/pcnet/) | 0.9.3 |
| Ethernet | VMware vmxnet3 15ad:07b0 | `capsule_driver_vmxnet3` | Works on QEMU: DHCP lease, batches | proof crate `vmxnet3_proofs` (22); QEMU lane `nic=vmxnet3`, [evidence](../release-evidence/0.9.3/drivers/vmxnet3/) | 0.9.3 |
| Ethernet | DEC Tulip 1011:0009, 1011:0019 (virtual machines) | `capsule_driver_tulip` | Works on QEMU: DHCP lease, batches | proof crate `tulip_proofs` (28); QEMU lane `nic=tulip`, [evidence](../release-evidence/0.9.3/drivers/tulip/) | 0.9.3 |
| Ethernet | Realtek RTL8126 10ec:8126, RTL8127 10ec:8127 | none | Not supported: left out of the RTL8169 driver, since their start differs from the RTL8125's | none | 0.9.2 |
| Ethernet | virtio-net 1af4:1000, 1af4:1041 | `capsule_driver_virtio_net` | Works | proof crate `virtio_net_proofs` (20); QEMU `virtio-net-pci` | 0.9.2 |
| Ethernet | USB CDC-ECM (class 02/06), CDC-NCM (02/0d), RNDIS (02/02/ff, e0/01/03, ef/04/01) | `capsule_driver_cdc_ecm`, `capsule_driver_cdc_ncm`, `capsule_driver_rndis` | Not supported: not in the image | proof crates `cdc_ecm_proofs` (7), `cdc_ncm_proofs` (33), `rndis_proofs` (22) | 0.9.2 |
| Ethernet | ASIX AX88179 and AX88178A, USB 0b95:1790, 0b95:178a | none | Not supported: no driver. Earlier releases listed a `capsule_driver_ax88179` that never existed in the tree | none | 0.9.3 |
