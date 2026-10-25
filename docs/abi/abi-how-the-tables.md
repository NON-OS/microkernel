# The NONOS ABI: How the tables are checked

Part of [The NONOS ABI](README.md).

## How the tables are checked

The numbers, names, gates, layouts, constants and ports in the tables on these
pages were generated from the source and compared with it by read-only scripts
kept outside the tree. The Meaning cells are written by hand from the handlers
and their comments.

The checks in the tree compare the files in `abi/` with the code.
`check_syscall_abi.py` also compares the capability each `[desc.TAG]` block in
`abi/syscalls.toml` publishes with the cap table. Run from the repository root,
each passes at this commit:

```sh
python3 scripts/check_abi_stable.py
python3 scripts/check_syscall_abi.py
python3 scripts/check_syscall_args.py
python3 scripts/check_caps_abi.py
```

## See also

- [Kernel syscalls](../kernel/syscalls.md)
- [Kernel capabilities](../kernel/capabilities.md)
- [The libc](../userland/libc.md)
- [The Linux personality](../userland/linux-personality.md)
- [The broker API for drivers](../drivers/broker-api.md)
- [Writing an app](../userland/writing-an-app.md)
- [abi/syscalls.toml](../../abi/syscalls.toml)
