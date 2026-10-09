# nonos_examples

Four small programs on `nonos_runtime`, each a separate crate with its own lock:

| Crate | What it does | Capability word passed to `nonos_main!` |
|---|---|---|
| `hello` | logs one line | CoreExec, Debug |
| `cli` | logs the uptime and four random bytes | CoreExec, Memory, Debug, Crypto |
| `gui` | creates a 256 by 256 surface and fills it | CoreExec, Memory and the four graphics bits |
| `service` | registers `example.echo` on port 5000 and echoes every message back | CoreExec, IPC, Memory, RegisterService |

The word only records what each program says it needs. None of them has a
`Capsule.mk`, so none is signed, enrolled or carried by an image. See
[the libc and std page](../../docs/handbook/userland/libc-and-std.md).
