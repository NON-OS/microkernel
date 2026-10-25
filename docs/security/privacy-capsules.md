# Privacy: The capsules that hold Network

Part of [Privacy](privacy.md).

The kernel serves the network services only to a capsule holding the Network
capability (`NETWORK_SERVICES` in `src/services/registry/policy.rs`). These are
the capsules whose `Capsule.mk` asks for it. The boot menu's `Air-Gapped`,
`Safe Mode` and `Recovery` take it from all of them.

| Capsule | Why it holds Network | In the desktop image |
|---|---|---|
| `capsule_net_core` | The network stack: DHCP, ARP, IP, TCP, UDP and lookups | yes |
| `capsule_net_sockets` | The socket service programs open connections through | yes |
| `capsule_net_nym` | The Nym mixnet client | yes |
| `capsule_net_anon` | The Anyone onion client | yes |
| `capsule_socks5` | The SOCKS5 front of the mixnet, which reaches only `net.nym` | yes |
| `capsule_browser` | Fetches the pages you open | yes |
| `capsule_terminal` | `curl`, `git`, `ping` and the market commands you type | yes |
| `capsule_wallet_nonos` | Reads balances and sends what you sign, over Nym or Anyone | yes |
| `capsule_shield` | The NOX Shield's pool and chain requests, over Nym or Anyone | yes |
| `capsule_model_fetch` | Downloads a Qwen model you ask for | yes |
| `capsule_audio_player` | Downloads a song you ask Music for | yes |
| `capsule_settings` | Reads the DHCP lease and Wi-Fi state to show them | yes |
| `capsule_desktop_shell` | Reads the DHCP lease for the top bar's online mark | yes |
| `capsule_linux` | Its install role, and its `run.net` role for a program whose policy asks for Anyone | yes |
| `capsule_net_l2` | The older stack's link layer; not built with `net_core` | no |
| `capsule_net_ip` | The older stack's IP layer | no |
| `capsule_net_udp` | The older stack's UDP | no |
| `capsule_net_tcp` | The older stack's TCP | no |
| `capsule_net_dhcp` | The older stack's DHCP client | no |
| `capsule_net_dns` | The older stack's resolver | no |
| `capsule_net_ntp` | Clock sync, only in the `microkernel-net-ntp` build | no |
| `capsule_nox_settle` | The settlement worker, in no desktop build | no |
| `capsule_prove_fetch` | Fetches for the app store and market index, in no desktop build | no |
| `capsule_tokio_smoke` | A runtime test for one test build | no |

The kernel's own copy of each grant is in `src/userspace/<capsule>/spawn.rs`,
and `scripts/cap_audit.py --strict` holds it to `Capsule.mk`.

Linux programs get Network only through two of `capsule_linux`'s roles: the
install role, for the mirror a package comes from, and the `run.net` role, for
a program whose policy names the Anyone network (`NET_ANON`), which reaches out
only through the network you chose, never Direct
(`src/userspace/capsule_linux/roles.rs`, `run_net.rs`). Every other Linux run
starts without it.
