# A Linux program tries to get out of its box, under KVM

The sealed 0.9.3 standard image, booted under KVM at 2560x1440 with no TPM. Setup was run with the Anyone
route and the Canyon real soft wallpaper, the Terminal was opened full window and `theme glass` was set.
Every command below was typed into the Terminal; `linux <program>` runs a Linux program in the Linux
personality. The programs have no listing of their own, so they ran in the default box. Each screenshot is
the Terminal after the command finished.

The standard image keeps the serial port quiet, so the `[LINUX]` lines are on the Terminal and not in a
serial log. The screenshots are the record.

Before the run, `write wallet-seed.txt abandon ability able about above absent absorb abstract` made a
file in the person's home, and `cat wallet-seed.txt` read it back. The words are a test string, not a
wallet.

| | Typed | What came back |
|---|---|---|
| [1](01-steal.jpg) | `linux cat /home/nonos/wallet-seed.txt` | `cat: can't open '/home/nonos/wallet-seed.txt': No such file or directory`, then `[LINUX] process 2 exited, status 1` |
| [2](02-hunt.jpg) | `linux find / -name '*wallet*'` | Only `find: /proc/2/exe: No such file or directory` and the same for `/proc/2/task/2/exe`. No file named for a wallet |
| [3](03-keys.jpg) | `linux ls -la /home/nonos/.ssh /home/nonos/.config` | `No such file or directory` for both |
| [4](04-procs.jpg) | `linux ls /proc` | `2` and the fixed files `cpuinfo filesystems loadavg meminfo mounts self stat sys thread-self uptime version`: one process, its own |
| [5](05-spy.jpg) | a `perl` one-liner calling `ptrace` (syscall 101) on pid 1 | `[LINUX] policy bound: from=default … mask=0x148 served=0x148 FILES_HOME EXEC TTY`, then `[LINUX] refused ptrace: a guest does not inspect or steer another` and `Operation not permitted` |
| [6](06-keylog.jpg) | `linux cat /dev/input/event0` | `cat: can't open '/dev/input/event0': No such file or directory` |
| [7](07-root.jpg) | a `perl` one-liner calling `setuid(0)` | It printed `setuid 0: root`. Not refused. The call succeeded inside the program's own box, where root has nothing outside it to reach |
| [8](08-exfil.jpg) | a `perl` one-liner connecting to 1.1.1.1 port 443 | `[LINUX] refused an internet socket: its policy does not grant NET_ANON, NET_LISTEN_LO or SERVE_ONE`, then `Bad file descriptor` |
| [9](09-audit.jpg) | `audit` | The table of every process and what it is granted |

A probe that tried to load a kernel module did not run: the one-liner had a Perl syntax error, so it proves
nothing and is left out.

These are single attempts from inside a program. They are not an exploit chain, and a program granted a
right is allowed it. The refused program launch (`refused execve: its policy does not grant EXEC`) comes
from Telegram Desktop's first start, which runs in the box its listing names. It is not in this run. The
developers saw it in their own run on a Mac under QEMU's software emulation, with these lines, and the
full log was not kept:

```
[LINUX] policy bound: from=market ... mask=0xb served=0xb WAYLAND NET_ANON FILES_HOME
[LINUX] refused execve: its policy does not grant EXEC
[LINUX] process 17 exited, status 255
```
