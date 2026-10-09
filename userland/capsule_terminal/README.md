# capsule_terminal

## Role

`capsule_terminal` is the NONOS shell: a desktop app capsule with up to nine
tabs, a scrollback with a VT emulator (`nonos_vt`), a line editor and a
command language whose commands are built into the capsule. It also starts
external programs, the bundled crates.io tools, `qwen` and the Linux
personality among them, and runs them as foreground or background jobs with
the keyboard as their stdin. Parsing, the builtins and job control all run
inside this one capsule; there is no separate shell process. The handbook
page is [docs/handbook/apps/terminal.md](../../docs/handbook/apps/terminal.md).

```text
desktop shell -- launch app.terminal --> terminal (nonos_app_skeleton)
terminal -- MkSurface* --> compositor
terminal -- MkIpc --> vfs_pool, installer, clipboard, net.*
terminal -- MkToolRun --> tool.NAME, tool.linux, tool.qwen, tool.model-fetch
terminal -- MkProcInput / MkProcOutput / MkTtySet --> its child jobs
```

## Microkernel contract

- Service `service:4722:app.terminal`, reply
  `reply:4723:endpoint.app.terminal.reply`. Three more windows can open on the
  instance endpoints `app.terminal.1` to `app.terminal.3` (ports 4740 to 4745).
- `CAPSULE_REQUIRED_CAPS = 0xc000187d`: CoreExec, Network, IPC, Memory, Crypto,
  FileSystem, GraphicsDisplayQuery, GraphicsSurfaceCreate, AppInstall and
  AttestRead. AppInstall is what `market install`, `market uninstall`,
  `market list` and `market info` take to ask the system for an install or a
  removal and to read where it stands.
  Network is what the network services and `MkToolRun` check for the network
  commands and the model fetcher; FileSystem is what vfs serves; Crypto backs
  the TLS handshake; AttestRead lets `receipt` read the attestation registry
  with `MkAttestEntries`, and `capsules` read every capsule's capabilities.
  `whoami`, `version` and the fresh-tab splash read the terminal's own entry
  there to say who signed it, and say "unknown" with the reason when the
  kernel does not hand the registry over.
- The kernel mirror is `src/userspace/capsule_terminal`. The capsule is turned
  on by `nonos-capsule-terminal` in `microkernel-desktop-offline`, so every
  desktop image carries it.

## Commands

`exec` and the `nox` table hold the builtins: files (`ls`, `cat`, `cp`, `mv`,
`rm`, `find`, `tree`, `grep` and the rest, over vfs under the terminal's own
pid), shell (`echo`, `set`, `alias`, `history`, `theme`, `help`), jobs and
processes (`jobs`, `fg`, `bg`, `kill`, `ps`, `exec`, `run`), system (`about`,
`uptime`, `capsules`, `receipt`, `bench`), network (`ping`, `ifconfig`,
`nslookup`, `nym`, `http`/`curl`/`get`/`fetch`, `pull`, `push`, `git`) and
apps (`market`, `apps`, `install`, `pkg`, `qwen`). Statements chain with `;`,
`&&`, `||` and `&`; `<`, `>`, `>>` and pipes work, and ten builtins
(`grep`, `sort`, `uniq`, `cut`, `nl`, `wc`, `head`, `tail`, `tac`, `rev`) act
as filters over a pipeline's lines, taking the same flags as on their own;
`sort`, `uniq`, `cut`, `nl`, `tac` and `rev` also read the files named after
them. Only these read a pipe: another command after a `|` stops the pipeline
and says so. `help <command>` has a page for every command `help` lists, and
`terminal_line_proofs` holds the two lists together.

## Launchpad tool tiles

A tool tile in the Launchpad hands the Terminal a command line through the
shell's `OP_TAKE_OPEN_ARG`, the call the editor uses to take a path
(`term/handed/`). The reply is `run:<line>` (run it as typed) or
`type:<line>` (leave it on the prompt, cursor at the end, for its
arguments). No path starts with either word, and anything else is ignored,
as is a line with a control byte in it. The window asks on every tick for
its first 3 s, so a command from the tile that launched it shows up with the
window. After that it asks twice a second, which covers a tile clicked
while the Terminal is open and raised. The command goes into the untouched
tab of a fresh window, or a new tab otherwise. A run goes through `on_enter`
like a typed line, so it is echoed, kept in history and its output lands in
that tab. Only the shell can answer the call (`desktop_shell` is a name no
other capsule may register), and the shell answers a command only to a
Terminal window and only one a Launchpad tile left.

## Network route

`http` and its aliases, and `git`, connect through `nonos_route_link`'s
`Route::chosen`: Nym through `net.socks5`, Anyone through `net.anon`, a direct
socket only when Direct is the system's default network, and no connection
with the reason when the chosen network is not running. `ping`, `nslookup` and
`pull` cannot cross an anonymity network and refuse unless Direct is chosen.
`push` is not gated: it resolves through `net.dns` and connects over `net.tcp`
directly whatever network is chosen.

## Privacy and persistence

Theme and font zoom are kept in `/etc/terminal/prefs.dat` through vfs.
History, aliases, variables and scrollback live in capsule memory and go with
the tab. A line that starts with `qwen` is the local model's question: it is
sent to the model's stdin and kept out of history. With no tier named, `qwen`
runs the tier the policy store holds (`Field::QwenTier`, chosen at setup or in
Settings), asked afresh on every `qwen` command so a change in Settings takes
effect at the next one.

## Failure model

A refused network route, a missing service or a failed child prints one line
in the scrollback and sets `$?`; it never panics the capsule. Closing a tab or
the window sends SIGTERM to every program that tab started.

## Tests

`userland/terminal_line_proofs` compiles the parser, expansion, filters, the
help text, the `qwen` line handling and the direct-only gate on the host;
`pipe_filter_tests` holds the pipe filters to their flags and
`help_truth_tests` holds `help` to the commands and the identity lines to the
registry. The
host tests under `tests/` cover layout, the rail and `pull` framing. Each is
one file built with `rustc --edition 2021 --test tests/<name>.rs`, as its
header says; `context_host` also takes `nonos_policy_proto` built as an rlib
and handed over with `--extern`.
