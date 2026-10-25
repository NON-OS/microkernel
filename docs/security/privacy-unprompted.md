# Privacy: What leaves without your asking

Part of [Privacy](privacy.md).

These go out on a desktop boot with the network on, before you open anything.
Each address is fixed in the image and is dialled without a name lookup.

## The address request

| Address | Who | Why | How to turn it off |
|---|---|---|---|
| `255.255.255.255` | `net_core` (and the older `net_dhcp`); on Hyper-V, `driver_netvsc` first | DHCP asks the local network for an address | `Air-Gapped` boot |

The request comes from a random, locally administered hardware address drawn
this boot, on wired cards as on Wi-Fi (`nonos_mac`). It carries no host name.
Its client id is that same per-boot address (smoltcp's DHCP client in
`net_core`; the older `net_dhcp` sends none, `dhcp/constants.rs`).

On Hyper-V, the network driver (`driver_netvsc`) sends up to three DHCP
DISCOVERs itself, two seconds apart, before the link is reported up, to learn
whether the host carries the drawn address: Hyper-V drops frames from any
address but the one it assigned unless the VM's MAC spoofing is on. Each comes
from the drawn address, to the broadcast address, with option 53 alone and no
client id or host name (`capsule_driver_netvsc/src/probe.rs`). If nothing
answers, the driver uses the address the host assigned the VM, which the host
knows already but the local network then sees, and says so on the console.
Turn MAC spoofing on for the VM's adapter to keep the drawn address
(`capsule_driver_netvsc/README.md`).

## The Nym mixnet client

| Address | Who | Why | How to turn it off |
|---|---|---|---|
| `92.39.63.14`, `validator.nymtech.net` | `net_nym` | The directory of gateways and mix nodes, over TLS on port 443 | `Air-Gapped` boot |
| `188.172.228.15`, `172.232.103.199`, `159.89.174.132` | `net_nym` | Gateways dialled until the directory arrives, port 9000 | `Air-Gapped` boot |
| `213.226.71.95`, `152.53.92.255` | `net_nym` | The same, the last two of five | `Air-Gapped` boot |
| `95.141.37.15`, `45.249.91.121`, `194.182.189.84` | `net_nym` | Layer-one mix nodes, for routes made before the directory | `Air-Gapped` boot |
| `54.38.81.30`, `207.148.68.94`, `162.55.174.117` | `net_nym` | Layer-two mix nodes, the same | `Air-Gapped` boot |
| `15.235.208.197`, `23.128.116.127`, `152.53.236.228` | `net_nym` | Layer-three mix nodes, the same | `Air-Gapped` boot |

The directory's own address is pinned in `directory_sync/pinned.rs` and
`directory_sync/live.rs`; the gateways are in `state/bootstrap.rs`, the mix
nodes in `state/bootstrap_mix/table.rs`. This machine never connects to a mix
node: its packets are wrapped for them and handed to the gateway, which sends
them on. After the directory arrives, the client dials the gateways it names,
which this list cannot know ahead. The client's identity is made new each boot
and kept nowhere.

## The Anyone client

| Address | Who | Why | How to turn it off |
|---|---|---|---|
| `49.13.145.234`, `5.161.108.187`, `5.78.90.106` | `net_anon` | Directory authorities: the consensus over HTTP, port 9230 | `Air-Gapped` boot |
| `5.161.228.187`, `5.78.94.15`, `95.216.32.105` | `net_anon` | The same | `Air-Gapped` boot |
| `176.9.29.53` | `net_anon` | The same, the seventh | `Air-Gapped` boot |

The list is `directory/authority/list.rs`; a consensus counts only with four of
the seven signatures. The client then builds circuits through relays the
consensus names. It keeps one guard for the boot and writes nothing, so no guard
survives a reboot.

## What is not yet in your hands

Both anonymity clients start on every boot that runs the network, under every
network choice, Direct included. No setting stops one without stopping the
network. Starting each only when the chosen network needs it is proposed, and
waits on the owner's decision.
