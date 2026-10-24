# Run a program in a box

Run a Linux program, one you did not write or one a model wrote, in a sealed box, and read a report of everything it tried that the box refused.

## Run it

Type `linux --box`, the box, then the program and its arguments, in the Terminal:

```
linux --box strict sh -c 'echo ok > /tmp/out; echo no > /home/out; nc 10.0.0.1 80'
linux --box offline python3 script.py
linux --box allow:/tmp/out,net:example.com:443 python3 fetch.py
```

The Terminal keeps eight words of a line, so after `linux --box <box>` a program has five words for itself and its arguments; give a script with `sh -c '...'` as one word.

## The boxes

| Box | Network | What it may write |
|---|---|---|
| `strict` | none | only `/tmp`, the run's own scratch folder |
| `offline` | none | the run's own folders: `/tmp`, `/home`, `/root`, `/run`, `/var/tmp`, `/dev/shm` |
| `allow:<item>,...` | only the destinations named | only the folders named |

An `allow:` item is a folder or a destination, with commas between and no spaces:

- A folder is a path from `/` inside one of the run's own folders, such as `/tmp/out` or `/home/work`. A folder anywhere else, a folder ending in `/`, or one with `.` or `..` in it is refused.
- A destination is `net:`, an IPv4 address or a name, `:` and a port from 1 to 65535: `net:10.0.0.1:443`, `net:example.com:443`. At most 64.

Every box lets the program use the terminal and start other programs. Each program it starts is held to the same box. The image itself, the Linux tree the programs come from, is always read-only.

A box only takes rights away. A program whose own policy gives it no network has none in any box; this is true of the shell and every program not listed with a policy of its own, so for them `net:` items reach nothing. The run's own folders are new at its start and wiped at its end.

A box named wrongly is refused before anything runs, with the reason: `linux: --box refused: a port is 1 to 65535`. `linux --box` with no box or no program prints the usage line.

## The report

When the program has ended, the Terminal shows its report. The report comes from `capsule_linux` on frames that carry a key the Terminal made for this run and the program is never told, so nothing the program prints is taken for the report, and it is drawn only after the program's output has ended and any half-written escape sequence it left has been cut off. A program can scroll the screen but cannot write, hide or change its own report.



```
BOX REPORT v1
  policy   strict 5811b081f826f154ce1ac5a7b558bff858ae0bacc27ad2db9d2e5f84ce05323a
  allowed  write /tmp
  allowed  read the image, read-only
  allowed  network none
  allowed  execve, each program held to this box
  refused  2
    write         1  /home/out
    network       1  an internet socket
  exit     0
  counts are capsule_linux's enforcement, not the kernel's sealed histogram
```

- `policy` is the box and its hash. Each box has one hash, whatever program runs in it and whatever order its items were named in; `strict` and `offline` always hash the same.
- `refused` is how many actions the box refused, then one line for each kind refused at least once: its count and the first three it refused.
- The kinds: `write` (a file written, made, renamed or removed outside the folders), `network` (a socket, a connection, a bind or a datagram the box does not allow), `exec`, `jit` (executable memory), `display`, `models`, `call` (a system call outside the program's declared calls), `barred` (calls no Linux program is ever given, such as `ptrace` and `mount`) and `unserved` (calls NONOS does not provide).
- One system call counts once for each kind it was refused as. Asking whether a file could be written, as `access` does, is not a refused action.
- `allowed` is what the program actually had: the box, narrowed by the program's own policy. A folder in `/home` or `/root` is listed only if the program's policy gives it a home, and `network` lists destinations only if its policy gives it network.
- `started  no: <reason>` appears when the program never started (it was not found, or its proof was refused); nothing ran, so nothing was refused.
- `exit` is the program's exit status.

If no report arrives, the Terminal says `linux: the box sent no report; it may have been stopped`. If the run is stopped with `Ctrl+C` before its report arrives, it says `linux: stopped before the box's report arrived`.

The counts are made by `capsule_linux` as it refuses each action. They are not the kernel's sealed histogram, which counts every call by the error it returned and is what a proof of a run reads.

## For automation

The Terminal also writes the same summary to the serial log as one line, in a build that compiles `capsule-serial-debug`:

```
[BOX] v=1 preset=strict policy=<hash> refused=2 write=1 network=1 exec=0 jit=0 display=0 models=0 call=0 barred=0 unserved=0 started=yes exit=0 source=capsule_linux
```

Every kind is always present. `v=1` is the format; a reader should refuse any other number.

## Where this comes from

- The boxes, the hash and the report: `userland/capsule_linux/src/linux/boxed/`.
- The report taken and drawn by the Terminal: `userland/capsule_terminal/src/jobs/box_hold.rs` and `box_draw.rs`.
- How a box narrows a program's own policy: `userland/capsule_linux/src/linux/app/identity_box.rs`.
- The tests: `userland/capsule_linux_proofs/src/tests/boxed/`, `userland/terminal_line_proofs/src/jobs/box_*`, and the fuzz target `box_word` in `userland/capsule_linux_proofs/fuzz/`.

## See also

- [Linux programs](linux-programs.md)
- [Terminal](terminal.md)
