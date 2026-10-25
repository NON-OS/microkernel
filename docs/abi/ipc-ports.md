# IPC: well-known service ports

Part of [IPC](ipc.md).

## Well-known service ports

These are the service endpoints declared in `userland/*/Capsule.mk`, each with the capsule that serves it and its reply port. `capsule_ports` in `scripts/capsule_port_sources.py` reads the same lines (`scripts/capsule_port_sources.py`, `def capsule_ports`), and `scripts/check_capsule_ports.py` finds no port declared twice at this commit. The table lists declarations; it does not say which capsules a given image carries. Window instance endpoints and the ports of Linux guests are left out.

| Port | Service | Reply port | Capsule |
|---|---|---|---|
| 4096 | `ramfs` | 4097 | `capsule_ramfs` |
| 4098 | `keyring` | 4099 | `capsule_keyring` |
| 4100 | `entropy_pool` | 4101 | `capsule_entropy` |
| 4102 | `crypto_pool` | 4103 | `capsule_crypto` |
| 4104 | `vfs_pool` | 4105 | `capsule_vfs` |
| 4106 | `market.index` | 4107 | `capsule_market` |
| 4108 | `policy` | 4109 | `capsule_policy` |
| 4110 | `wallpaper_catalog` | 4111 | `capsule_wallpaper_catalog` |
| 4112 | `installer` | 4113 | `capsule_installer` |
| 4114 | `payment` | 4115 | `capsule_payment` |
| 4200 | `driver.virtio_rng` | 4201 | `capsule_driver_virtio_rng` |
| 4202 | `driver.virtio_blk0` | 4203 | `capsule_driver_virtio_blk` |
| 4204 | `driver.virtio_net0` | 4205 | `capsule_driver_virtio_net` |
| 4206 | `driver.xhci0` | 4207 | `capsule_driver_xhci` |
| 4208 | `driver.ps2_kbd0` | 4209 | `capsule_driver_ps2_input` |
| 4210 | `driver.e1000_0` | 4211 | `capsule_driver_e1000` |
| 4212 | `driver.rtl8139_0` | 4213 | `capsule_driver_rtl8139` |
| 4214 | `driver.rtl8169_0` | 4215 | `capsule_driver_rtl8169` |
| 4216 | `driver.ahci0` | 4217 | `capsule_driver_ahci` |
| 4218 | `driver.hda0` | 4219 | `capsule_driver_hda` |
| 4220 | `driver.nvme0` | 4221 | `capsule_driver_nvme` |
| 4222 | `driver.usb_hid0` | 4223 | `capsule_driver_usb_hid` |
| 4224 | `driver.usb_msc0` | 4225 | `capsule_driver_usb_msc` |
| 4226 | `driver.virtio_gpu0` | 4227 | `capsule_driver_virtio_gpu` |
| 4228 | `driver.iwlwifi0` | 4229 | `capsule_driver_iwlwifi` |
| 4230 | `driver.i2c_pci0` | 4231 | `capsule_driver_i2c_pci` |
| 4232 | `driver.i2c_hid0` | 4233 | `capsule_driver_i2c_hid` |
| 4234 | `driver.rtl8821ce0` | 4235 | `capsule_driver_rtl8821ce` |
| 4250 | `driver.cdc_ecm0` | 4251 | `capsule_driver_cdc_ecm` |
| 4252 | `driver.cdc_ncm0` | 4253 | `capsule_driver_cdc_ncm` |
| 4254 | `driver.rndis0` | 4255 | `capsule_driver_rndis` |
| 4256 | `driver.ax88179_0` | 4257 | `capsule_driver_ax88179` |
| 4258 | `driver.rtl8153_0` | 4259 | `capsule_driver_rtl8153` |
| 4270 | `driver.e1000e_0` | 4271 | `capsule_driver_e1000e` |
| 4272 | `driver.igc_0` | 4273 | `capsule_driver_igc` |
| 4290 | `driver.rtsx0` | 4291 | `capsule_driver_rtsx` |
