# Large Linux installs on the heap fix

One TCG boot (4 CPUs, 8 GiB) of a dev image of gui at 32c71beb2b: main
with the heap fix (heap-bins, the same heap code as main's 1db754d517),
the installer's virtual-need fix, and boot-test recipes for KeePassXC and
gnome-authenticator. Three installs one after another in the Terminal:
swayimg, then KeePassXC, then gnome-authenticator.

| install | packages verified | wall time | per package |
|---|---|---|---|
| swayimg | 23 (with the boot's first index reads) | | |
| KeePassXC 2.7.8 (Qt 5, qt5-qtwayland by install_if) | 126 | 892 s | 7.1 s |
| gnome-authenticator 4.4.0 (GTK 4) | 155 | 1174 s | 7.6 s |

304 packages verified in one boot. No stall, no `[LOCK]` line, no
`[OOM]` or panic. Eight `[SMP] tlb shootdown slow` lines (one ack late
after 50 ms, answered by NMI).

- serial.log: the boot's serial log.
- verified-over-time.txt: every 10 s, the host clock, the count of
  "provenance Verified" lines, and of "[LINUX] installed" lines.

The time per package is the whole install under TCG from a mirror relay on
the host, so it is the emulated CPU's cost of verifying and unpacking, not
the network's.

Neither app's window opened in this boot: a run holds one store handle a
pack, and the store gave an owner 128 (fixed in a052e9a469); the next boot
shows the windows.
