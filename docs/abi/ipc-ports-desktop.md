# IPC: service ports 4310 to 4505

Part of [IPC](ipc.md).

Ports 4310 to 4505 of the [well-known service ports](ipc-ports.md#well-known-service-ports).

| Port | Service | Reply port | Capsule |
|---|---|---|---|
| 4310 | `compositor` | 4311 | `compositor` |
| 4320 | `input_router` | 4321 | `capsule_input_router` |
| 4330 | `wm` | 4331 | `capsule_wm` |
| 4340 | `wallpaper` | 4341 | `capsule_wallpaper` |
| 4400 | `net.l2` | 4401 | `capsule_net_l2` |
| 4402 | `net.ip` | 4403 | `capsule_net_ip` |
| 4410 | `desktop_shell` | 4411 | `capsule_desktop_shell` |
| 4412 | `image_codec` | 4413 | `capsule_image_codec` |
| 4414 | `clipboard` | 4415 | `capsule_clipboard` |
| 4416 | `login` | 4417 | `capsule_login` |
| 4420 | `net.udp` | 4421 | `capsule_net_udp` |
| 4430 | `net.tcp` | 4431 | `capsule_net_tcp` |
| 4440 | `net.dhcp.client` | 4441 | `capsule_net_dhcp` |
| 4444 | `attest` | 4445 | `capsule_attest` |
| 4446 | `power.off` | 4449 | `capsule_power`, its second name: the shell's wiped power-off |
| 4448 | `power` | 4449 | `capsule_power` |
| 4450 | `net.dns` | 4451 | `capsule_net_dns` |
| 4460 | `net.sockets` | 4461 | `capsule_net_sockets` |
| 4470 | `net.nym` | 4471 | `capsule_net_nym` |
| 4480 | `net.core` | 4481 | `capsule_net_core` |
| 4482 | `net.ntp.client` | 4483 | `capsule_net_ntp` |
| 4484 | `net.anon` | 4485 | `capsule_net_anon` |
| 4500 | `proof_io` | 4501 | `capsule_proof_io` |
| 4502 | `std_proof` | 4503 | `capsule_std_proof` |
| 4504 | `tokio_smoke` | 4505 | `capsule_tokio_smoke` |
