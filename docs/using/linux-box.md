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

When the program ends, the Terminal shows:

```
BOX REPORT v1
  policy   strict ab9fb0d1cd685a85f0db55034e43c52ba5d0cbff31b8f2b3050bf01581bc6f67
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
- `exit` is the program's exit status.

The counts are made by `capsule_linux` as it refuses each action. They are not the kernel's sealed histogram, which counts every call by the error it returned and is what a proof of a run reads.

## For automation

The same summary goes to the serial log as one line:

```
[BOX] v=1 preset=strict policy=<hash> refused=2 write=1 network=1 exec=0 jit=0 display=0 models=0 call=0 barred=0 unserved=0 exit=0 source=capsule_linux
```

Every kind is always present. `v=1` is the format; a reader should refuse any other number.

## Where this comes from

- The boxes, the hash and the report: `userland/capsule_linux/src/linux/boxed/`.
- How a box narrows a program's own policy: `userland/capsule_linux/src/linux/app/identity_box.rs`.
- The tests: `userland/capsule_linux_proofs/src/tests/boxed/`, and the fuzz target `box_word` in `userland/capsule_linux_proofs/fuzz/`.

## See also

- [Linux programs](linux-programs.md)
- [Terminal](terminal.md)
