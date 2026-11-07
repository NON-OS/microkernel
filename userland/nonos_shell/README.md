# nonos_shell

The NØNOS terminal's command **language**, carved out of `capsule_terminal`
into a host-agnostic library so a headless `nsh` session and the local GUI
terminal run the **same** parser, the same statement/redirect grammar, the same
route tables and the same pipe filters. `no_std` + `alloc`, no GUI, no
scrollback, no syscalls.

The first wave moved the portable language core. The second moved the
builtins that need nothing of the terminal, added a `Host` trait for what a
builtin asks of the machine, added the builtins an nsh session needs, and
added a headless session with control flow, which nsh.front now runs. The
builtins tied to the terminal's `State` and the GUI stay in
`capsule_terminal`, which consumes this crate. See "What remains" below.

## Public modules

| Module | What it is |
|---|---|
| `parse` | Tokenize a line into words, quote-aware. `parse::parse`, `parse::Argv`. |
| `flags` | Short/long flag tables (`-n`, clustered, `--`). `flags::{parse, parse_usize, Spec, Parsed}`. |
| `expand` | `$NAME` / `$?` substitution over a caller-supplied var set + status. `expand::expand(line, vars, last_status)`. |
| `alias_expand` | Alias substitution over a caller-supplied alias set. `alias_expand::alias_expand(stmt, aliases)`. |
| `statements` | Split a line on `;`, `&&`, `||`, `&`, quote-aware. `statements::{split_program, Conn, Stmt}`. |
| `redirect` | The redirect plan (`<`, `>`, `>>`, `2>&1`). `redirect::{plan, is_operator, Sink, Source, Plan}`. |
| `tool_admit` | Whether a tool is admitted given its redirect plan. `tool_admit::{admit, hears_end_of_input}`. |
| `suggest` | "did you mean" nearest-command. `suggest::{nearest_two, distance}`. |
| `filter` | Pipe filters: `grep sort uniq cut nl wc head tail tac rev`, over a pipe, a redirected file, or named files. `filter::{apply, count, input, text}`. |
| `route` | Which command each name means: the four group tables (`Info`, `Session`, `Files`, `Net`) + the cross-table lookup. `route::find::{route, Shell}`. |
| `out` | `ShellOut`: the output sink a caller plugs in. |
| `host` | `Host`: what a builtin asks of the machine (randomness, wall clock, uptime, memory, the process table, the boot's measurement, reading a file). Every answer may be refused. |
| `builtin` | The shared builtins: `echo printf seq hex b64 hash rand` (moved from the terminal, byte for byte in behaviour), and `basename dirname diff date uptime mem ps ed25519 test` (new). |
| `session` | The headless session: `Session::run_line(line, out, host)`, with `if`/`for`/`while`, the shared builtins, the pipe filters, the session's own variables and aliases (`alias`, `unalias`, each held to 1024 bytes and 64 a session), `type`/`which`, `history` (the last 64 lines, 4096 bytes of each), `clear`, `version` (the release, from VERSION), `attest` and `pcr` (the measured-boot state the kernel recorded, as the terminal prints it), `commands` (the same list as `help`), and `whoami`, which prints the login nsh.front admitted the session as (user, role, key digest). |
| `util` | Two pure leaf helpers (`is_space`, `format_u64`). |

Everything here is a byte-for-byte move of the capsule's former
`command::{parse, flags, suggest}` and `command::dispatch::{expand,
alias_expand, statements, redirect, tool_admit, filter, route}`; the only edits
were re-homing two leaf helpers (`crate::util`) and the flag module path
(`crate::flags`), and widening four `pub(crate)`/`pub(super)` items to `pub` so
the capsule can reach them across the crate boundary (`route::find::{Shell,
route}`, the four group enums, `filter::apply`).

## The output coupling: `out::ShellOut`

The one environment coupling the language exposes is **where output lines go**:

```rust
pub trait ShellOut {
    fn writeln(&mut self, line: &[u8]); // one line; the sink adds its own terminator
}
```

- The GUI terminal implements it over its scrollback `Output`
  (`capsule_terminal/src/command/output/shell_out.rs`): `writeln` pushes a line
  into the scrollback ring, exactly as `Output::writeln` always did.
- A headless `nsh` session implements it over its session byte stream.
- `out::Sink` is a no-op sink (status without text, and for tests).

This mirrors the capsule's existing `Output::writeln(&[u8])` one-to-one, so a
builtin migrated from `&mut Output` to `&mut dyn ShellOut` prints the same bytes
in the same order. The shared builtins take `&mut dyn ShellOut`; the terminal
passes its `Output`, an nsh session passes `out::Crlf`, which ends each line
with `\r\n` for the terminal at the other end.

## Headless consumption contract (what nsh adopts)

A headless caller keeps one `session::Session` per connection and hands it
each typed line with an output sink and a `Host`:

```rust
let mut out = nonos_shell::out::Crlf::default();
match session.run_line(line, &mut out, &mut host) {
    Flow::Next => send(&out.0),
    Flow::Exit(code) => end(code),
}
```

`run_line` splits the line into statements (`;`, `&&`, `||`), reads them into
control flow (`if`/`then`/`else`/`fi`, `for NAME in WORDS; do ...; done`,
`while ...; do ...; done`, nested to 16 deep), and runs each simple command
when it is reached: aliases, then `$NAME`/`$?`, then words, then one builtin,
or a builtin whose lines pass the filters after each `|`. A `while` stops
after 10 000 rounds and a line after 100 000 commands. Redirects and
background jobs are refused, since a session holds no filesystem and no job
table. nsh.front's shell (`capsule_nsh_front/src/shell.rs`) is this loop.

The GUI terminal still runs its own spine (`command::dispatch`), which calls
the shared builtins through the same names.

## Capability surface

A shared builtin reaches the machine only through `Host`, and every `Host`
answer may be None or false. A builtin whose host refuses prints why and
fails; it never assumes a grant. The terminal's `Host`
(`capsule_terminal/src/command/host.rs`) answers from the terminal's own
capabilities. nsh.front's (`capsule_nsh_front/src/shell_host.rs`) answers
randomness, the clocks, memory and the process table, and refuses files,
since it holds no filesystem capability. The kernel hands a process without
AttestRead or ProcessControl each process's pid, parent, state and name with
its counters cleared, which is all `ps` prints.

The builtins still in the terminal reach the kernel through `nonos_libc`
under the terminal's manifest (`capsule_terminal/Capsule.mk`): FileSystem
(`ls cat cp mv rm mkdir ... cd`), Network (`ping`, `nslookup`,
`nox http/pull/push`, `nym`), AppInstall (`market`), SpawnWindow (`nox install`,
`tool`) and AttestRead (`attest`, `receipt`).

## Proof coverage

- `nonos_shell_proofs`: the shared builtins, moved and new, through a
  recording sink and a scripted host (including RFC 8032 test 1 for
  `ed25519`), and the session: gating, each control-flow form, both bounds,
  pipes, every syntax refusal, `exit`, names and aliases.
- `nsh_server_proofs` `session_shell`: the real client logged in to the real
  server, lines run through the session as nsh.front wires it, the client's
  output and the record checked.
- `terminal_line_proofs` (369) and `terminal_host_proofs` mount this crate's
  files by `#[path]` and still pass. The `line-proofs` feature exposes
  `route::find::names` for the route-table proofs.

## What remains for full extraction

- **The builtins tied to `State`.** `fs`, `ping`/`nslookup`, `ps`, `nox`,
  `market`, `qwen`, `bench`, `git`, `jobs` and the info builtins still take
  the terminal's `State` or reach `nonos_libc` directly. Each needs the `Host`
  methods its capability implies (directory listing, stat, sockets) before it
  can move. An nsh session would also need the capability itself: nsh.front
  holds none of FileSystem, AppInstall or SpawnWindow, and adding one changes
  its measured capability mask.
- **The terminal's dispatch spine.** `command::dispatch::{exec, run,
  pipeline, write_redirect}` stays in the terminal: it writes redirects
  through the VFS and runs jobs, which the session refuses. The terminal does
  not run the new builtins or `if`/`for`/`while` yet; routing them there
  needs the route table, completion and help entries, and the line proofs
  that pin them.

## Nix inputs

This crate's path dependencies (`nonos_hash`, `nonos_base64`,
`nonos_ed25519`) and the new mounts reach `tools/nix/inputs.json`, which is
regenerated by `python3 tools/nix/inputs.py`. That file already differed from
the tree before this wave; the seal writes it again.
