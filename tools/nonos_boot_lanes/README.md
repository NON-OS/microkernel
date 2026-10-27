# the boot lanes for 0.9.3

Steps for `tools/nonos-drive`, one file per lane, on a development image
(`tools/nonos-dev-image --profile standard`). Each names what it shows and
what a pass looks like. Run with the Intel IOMMU lane, as every S1 lane is:

    tools/nonos-drive --image <img> --out <dir> --qemu-arg=--lane --qemu-arg=iommu=intel \
      --qemu-arg=--fresh --steps tools/nonos_s1_lanes/<lane>.steps

| Lane | Extra QEMU lanes | What it shows | A pass |
|---|---|---|---|
| `coherence.steps` | none | A file mapped MAP_SHARED, as LMDB does: a write() seen through a read map, a write through a writable map seen by read(), both kept for the next run | the Terminal (`coherence.png`, `next-run.png`) shows `P ro ok`, `P seen HELLO BEYOND`, `P over refused`, `P rw ok`, `P read WORLD`, `P msync 0 unmap 0`, `P exec refused`, then `P next_run HELLO WORLD BEYOND size 9006`; the typed line stays under the Terminal's input limit |
| `monero.steps` | needs the network: the dev relay, or a machine whose QEMU user net reaches the internet | Monero's ring database (LMDB, mapped shared): a ring set in one run of the wallet and printed in the next | no "Failed to initialize ring database"; `print_ring` shows the ring in both runs; `/root/.shared-ringdb` holds `data.mdb` and `lock.mdb` |
| `monero-sync.steps` | after the host's network-enabled setup steps; needs real internet | The Monero wallet reaching a node through the SOCKS endpoint (127.0.0.1:9050) on the network setup chose (Nym or Anyone) | `status` shows the daemon's height; `refresh` scans blocks; no 'Failed to connect to daemon'. On Anyone, ports 18081/18089 need anon-exit-ports (exits chosen by port) |
| `group.steps` | `--lane hda-beside-nic --lane nic=e1000e` | Two vendor drivers' devices in one isolation group, as the HP's Wi-Fi sits beside its NVMe disk | serial shows `[VT-D] shared: ... each in its own domain`, and both the HDA and the e1000e drivers come up; a hardened boot shows `[VT-D] refused:` instead |

Every lane also counts `[LOCK]`, unanswered calls and traps in `serial.log`;
all three should be 0.
