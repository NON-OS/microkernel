# lock runs, booted 27 October 2026 (macOS host, QEMU TCG, no KVM)

Kernel under test: nonos/jolly-gauss-xbkfum at c3f5225753 (adb53a20d ticket lock and
lock-free asid directory; abae61bc9 per-guest peer stripes; main f68bb8ac3 merged).

- (i) a1, a2: the branch as is, 4 CPUs, `--lane iommu=intel`, `--fresh`, 8 GiB.
  a3: the same at 8 CPUs, no iommu lane.
- (ii) b1, b2, b3: the branch plus `git cherry-pick 0448b98c3` (MkGetPid), same lanes.
  bm: a (ii) boot, 4 CPUs, iommu=intel, fresh, with `market install linux.monero` from a
  dev mirror relay, then `linux monero-wallet-cli --version` twice (status 0 each;
  the first run went through the network restart, app.linux.term to app.linux.term.net).

Each boot is `make dev-image PROFILE=standard` with throwaway keys (never a release),
first-boot setup driven by keys through QMP, then 3 minutes on the desktop (a1 stopped
at shell_start). Counts are `grep -c` over each serial log; summary.txt has them.

| lane | CPUs | iommu | desktop at (ms) | [LOCK] | unanswered |
|---|---|---|---|---|---|
| a1 (i) | 4 | intel | 170999 | 0 | 0 |
| a2 (i) | 4 | intel | 172185 | 0 | 0 |
| a3 (i) | 8 | none | 171991 | 0 | 0 |
| b1 (ii) | 4 | intel | 171181 | 0 | 0 |
| b2 (ii) | 4 | intel | 172051 | 0 | 0 |
| b3 (ii) | 8 | none | 171943 | 0 | 0 |
| bm (ii) + Monero | 4 | intel | see log | 0 | 0 |

Under TCG this says nothing about KVM or real hardware timing.
