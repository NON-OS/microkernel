# AgentProof box policies: the `nonos-box 1` file

A box policy is a short text file that says what a program run in an AgentProof box may
write and which destinations it may connect to. `agentproof policy check` reads it, refuses it with the line and the reason, or says in
plain words what it allows, with the `linux --box` word it means and that box's policy hash.
The hash is the one the run's `BOX REPORT` shows, because the word is checked and hashed by
`capsule_linux`'s own code, not a copy of it.

## An example

```
nonos-box 1
# a build agent: it writes its output and a work folder, nothing else
base folders
write /tmp/out
write /home/work
network none
```

```
$ agentproof policy word build.box
allow:/home/work,/tmp/out
```

Run the program in it with `linux --box allow:/home/work,/tmp/out python3 build.py`.

## The lines

- The first line that is not blank or a comment is `nonos-box 1`, the format's version.
  A reader refuses any other version.
- `base strict`: the run's own `/tmp` only. The same box as `linux --box strict`.
- `base offline`: the run's own `/tmp`, `/home`, `/root`, `/run`, `/var/tmp` and `/dev/shm`.
  The same box as `linux --box offline`.
- `base folders`: only the folders the `write` lines name; at least one is needed.
- `write <folder>`: a folder from `/`, inside one of the run's own folders above, in printable
  ASCII, with no `.`, `..` or empty part and no `/` at its end. At most 16, each named once.
  Only with `base folders`.
- `network none`: no network. A file has either this line or `connect` lines.
  `network` with anything but `none` is refused: `network is \`none\`; name each destination
  on a connect line`.
- `connect <address or name>:<port>`: a destination the program may open a TCP connection
  to: an IPv4 address whole, or a name, and one port from 1 to 65535. At most 64, each named
  once (`API.example.com:443` and `api.example.com:443` are one), with no comma. Only with
  `base folders`, and never with `network none`. Each becomes a `net:` item of the box word.
- A line whose first word starts with `#` is a comment. Blank lines are skipped.

Words are separated by spaces. Lines may come in any order after the header. `base` and
`network` are each named once; a file names `network none` or at least one `connect` line. A file is printable ASCII, spaces and newlines, at most 16384
bytes; a tab or carriage return is refused. Any other line is refused, never skipped. The
box word a file makes is at most 255 bytes.

## A box with destinations

```
nonos-box 1
base folders
write /tmp/out
connect api.example.com:443
connect 10.0.2.2:80
```

```
$ agentproof policy word api.box
allow:/tmp/out,net:api.example.com:443,net:10.0.2.2:80
```

A box naming destinations runs in the personality's one terminal role that holds the
network (`app.linux.term.net`); every other box runs in a role that has none. In it:

- A program may open TCP connections to the destinations listed and to no other. A
  connection to anything else, and any UDP datagram, is refused and counted as `network`.
- Each connection goes over the anonymity network the system uses (the Nym mixnet, or
  Anyone when it is chosen), never directly, whatever the default network is.
- A name is answered by NONOS itself; the lookup never leaves the machine.
- A program with no policy of its own gets the network the box names. A program with a
  published policy gets it only if its own policy grants the network, and is then held to
  its own destinations and the box's both; a box never widens a published policy. A program with a published policy and no network is not started in a
  box naming destinations: `the network role runs only a program granted NET_ANON`.
- Each program the run starts is held to the same list.

The report lists the destinations the box allowed and each one the run connected to, and
a `[BOX-NET]` line before the `[BOX]` line logs both (see
[Run a program in a box](../using/linux-box.md)).

## What every box allows

Every box gives a terminal and execve, and each program the run starts is held to the same
box. The image is read-only. A box without `connect` lines has no network. A program gets
what both the box and its own policy allow, so a folder in `/home` or `/root` is writable
only when the program's own policy gives it a home. The run's folders are new at its start and wiped at its end. See
[Run a program in a box](../using/linux-box.md).

## The command

```
agentproof policy check <file>   what it allows, in plain words, with its word and hash
agentproof policy word <file>    the box word only
agentproof policy hash <file>    the policy hash only
```

It exits 0 for a policy, 1 when the file is refused (`agentproof: refused: <file>:<line>:
<reason>`, or no line when something is missing), and 2 when the command is wrong or the file
cannot be read. It builds on the host: `cargo build --release` in `userland/agentproof_policy`.

## What this does not claim

The counts in a box's report are capsule_linux's enforcement, not the kernel's sealed record.
A policy file is not signed and is not carried in any receipt in this release.

## Where this comes from

- The format: `userland/agentproof_policy/src/format/`.
- The box, its rules and its hash, mounted from `userland/capsule_linux/src/linux/boxed/`.
- The proofs: `userland/agentproof_proofs`, its mutants (`mutants.py`) and fuzz target
  `policy_file`.
