# Errors

What a failed NONOS syscall returns, the codes the handlers use, and every errno name the kernel defines.

## How an error comes back

A call returns a signed 64-bit value in the return register. Zero or more is success. A negative value is an errno with its sign flipped: `SyscallResult::error` stores `-errno` (`src/syscall/types/result.rs:35-37`). The microkernel handlers return negative `i64` constants directly, as the header of their errno file says (`src/syscall/microkernel/errnos.rs:17-20`). So `EINVAL` arrives as -22. `abi/syscalls.toml` publishes `errno_negative` and an `errno_range` of -4095 to -1 (`abi/syscalls.toml:15-19`).

Two refusals come before any handler, and a third before most:

- An unknown number is `ENOSYS`, -38 (`src/arch/x86_64/syscall/manager/entry.rs:38-53`). On x86_64 a [foreign process](../overview/glossary.md#foreign-process) is the exception: its unknown number goes to its supervisor, which chooses the answer.
- A call the caller's [capabilities](../overview/glossary.md#capability) do not admit is `EPERM`, -1, from the contract `dispatch` (`src/syscall/contract/dispatch.rs:31-37`).
- A register too wide for the field it carries is `EINVAL`, -22, where the dispatch reads it with `u32_arg` or one of its siblings and refuses rather than truncates (`src/syscall/microkernel/narrow.rs:17-35`). The IPC calls are the exception: there `resolve_for_recv` and the send path treat an endpoint number too wide for a port as one that names no endpoint, and the call fails as for any unknown endpoint (`src/syscall/microkernel/ipc/inbox_name.rs:28-40`).

The names and numbers are those of Linux on x86_64. `posix.rs` says it holds the standard POSIX errnos 1 to 40 and the aliases `EWOULDBLOCK` and `EDEADLOCK` (`src/syscall/types/errnos/posix.rs:17`). Every value in the kernel's three errno files was compared with the errno table of a Linux x86_64 host and matches it.

## The codes handlers return

The microkernel handlers return these 24 constants, `ERRNO_PERM` to `ERRNO_STALE` (`src/syscall/microkernel/errnos.rs:22-54`). Published says whether `abi/syscalls.toml` lists the code in its `[errors]` table, which runs from `EPERM` to `ENOTEMPTY`; 10 of its 25 entries are not in this set (`abi/syscalls.toml:21-46`). Where the source gives a reason beside a constant, the meaning here is that reason.

| Constant | Name | Value | Published | Meaning |
|---|---|---|---|---|
| `ERRNO_PERM` | `EPERM` | -1 | yes | The caller lacks a capability, does not own the object, or is not its parent. A refusal by the capability check is the same value. |
| `ERRNO_NOENT` | `ENOENT` | -2 | yes | No such [inbox](../overview/glossary.md#inbox), service, file or record. |
| `ERRNO_IO` | `EIO` | -5 | yes | The volume or the disk under it could not do what was asked. |
| `ERRNO_CHILD` | `ECHILD` | -10 | yes | `MkWait` named a process that is not the caller's child. |
| `ERRNO_AGAIN` | `EAGAIN` | -11 | yes | No disk is chosen yet, because the USB driver is still looking, or a supervisor already hosts as many guests as it may; ask again. |
| `ERRNO_NOMEM` | `ENOMEM` | -12 | yes | Out of memory, address space, slots or table room. |
| `ERRNO_ACCES` | `EACCES` | -13 | yes | Access refused, for example a receive on an [endpoint](../overview/glossary.md#endpoint) another process owns. |
| `ERRNO_FAULT` | `EFAULT` | -14 | yes | A user pointer is not mapped, or not writable where the kernel must write; from the store calls, a device fault. |
| `ERRNO_BUSY` | `EBUSY` | -16 | yes | Already claimed or registered, or the target inbox is full. |
| `ERRNO_EXIST` | `EEXIST` | -17 | yes | The endpoint or volume to create already exists. |
| `ERRNO_NODEV` | `ENODEV` | -19 | yes | No such device, or the device or service behind the call is gone. |
| `ERRNO_INVAL` | `EINVAL` | -22 | yes | An argument is out of range, or wider than its field. |
| `ERRNO_NOTTY` | `ENOTTY` | -25 | yes | `MkTtyQuery` asked about a stream that is not on a terminal. |
| `ERRNO_FBIG` | `EFBIG` | -27 | no | A stream fed more bytes than the length named at its start. |
| `ERRNO_NOSPC` | `ENOSPC` | -28 | no | The volume has no room for the bytes still to come. |
| `ERRNO_RANGE` | `ERANGE` | -34 | yes | A DMA map whose device address a 32-bit descriptor cannot carry. |
| `ERRNO_NOSYS` | `ENOSYS` | -38 | yes | Not available here: port I/O and the foreign-process calls on aarch64 and riscv64. |
| `ERRNO_BADMSG` | `EBADMSG` | -74 | no | The file to import is not the one its digest pins. |
| `ERRNO_NOTSUP` | `EOPNOTSUPP` | -95 | no | Unsupported flags or mode, or a battery charge the kernel cannot read. |
| `ERRNO_NETDOWN` | `ENETDOWN` | -100 | no | A network tool was asked for, and the [boot profile](../overview/glossary.md#boot-profile) runs no network: Air-Gapped, Safe Mode or Recovery. |
| `ERRNO_TIMEDOUT` | `ETIMEDOUT` | -110 | no | A receive, call or wait ran out its timeout, or a block device did not answer. |
| `ERRNO_ALREADY` | `EALREADY` | -114 | no | The file was imported and verified before; nothing to feed. |
| `ERRNO_INPROGRESS` | `EINPROGRESS` | -115 | no | A stream finished before every byte came; what came is kept. |
| `ERRNO_STALE` | `ESTALE` | -116 | no | The [claim epoch](../overview/glossary.md#claim-epoch) is not the current one: the device was released and claimed again. |

Some calls map their own failures onto these codes:

- `store_errno` gives `MSWR` and `MSRR` one table: no disk is `ENODEV`, a refused caller `EACCES`, a driver not ready `ETIMEDOUT`, a request outside the disk `EINVAL`, and the rest `EFAULT` (`src/syscall/microkernel/store_errno.rs:29-41`). `kernel_proofs` mounts `store_errno` on the host (`userland/kernel_proofs/src/store_errno/mod.rs:24`); its 388 host tests passed in the flake check run on this commit.
- The [broker](../overview/glossary.md#broker) calls map a missing claim to `EPERM`, a stale claim epoch to `ESTALE`, an unknown device to `ENODEV`, and unsupported flags to `EOPNOTSUPP`; `errno_for` is the DMA map (`src/syscall/microkernel/dma.rs:99-112`). See [Broker](broker.md).
- `map_capsule_error` turns a failure of the crypto [capsule](../overview/glossary.md#capsule) into `EACCES`, `EINVAL`, `EBADMSG` for a failed authentication, `EIO`, `EMSGSIZE`, `EPROTO`, `ENODEV` or `ESTALE` (`src/syscall/dispatch/crypto/error.rs:26-43`).
- `sys_time_rtc` and `sys_time_millis` return -61, `ENODATA`, written as a bare number, while the kernel has no wall-clock time yet (`src/syscall/microkernel/time.rs:33-36`, `src/syscall/microkernel/time.rs:55-58`).
- The surface and input calls keep their own copies of a few values, among them `ESRCH`, 3, when there is no calling process (`src/syscall/dispatch/router/surface_ops.rs:24-29`).

## Every errno the kernel defines

`src/syscall/types/errnos/` names 131 values and two aliases, `EWOULDBLOCK` and `EDEADLOCK` (`src/syscall/types/errnos/posix.rs:58-59`). Most are never returned by a native handler. The kernel defines names and numbers only; the meaning column is the text a Linux C library prints for the number.

| Name | Value | Published | Meaning |
|---|---|---|---|
| `EPERM` | -1 | yes | Operation not permitted. |
| `ENOENT` | -2 | yes | No such file or directory. |
| `ESRCH` | -3 | yes | No such process. |
| `EINTR` | -4 | yes | Interrupted system call. |
| `EIO` | -5 | yes | Input/output error. |
| `ENXIO` | -6 | yes | No such device or address. |
| `E2BIG` | -7 | yes | Argument list too long. |
| `ENOEXEC` | -8 | yes | Exec format error. |
| `EBADF` | -9 | yes | Bad file descriptor. |
| `ECHILD` | -10 | yes | No child processes. |
| `EAGAIN` | -11 | yes | Resource temporarily unavailable. |
| `ENOMEM` | -12 | yes | Cannot allocate memory. |
| `EACCES` | -13 | yes | Permission denied. |
| `EFAULT` | -14 | yes | Bad address. |
| `ENOTBLK` | -15 | no | Block device required. |
| `EBUSY` | -16 | yes | Device or resource busy. |
| `EEXIST` | -17 | yes | File exists. |
| `EXDEV` | -18 | no | Invalid cross-device link. |
| `ENODEV` | -19 | yes | No such device. |
| `ENOTDIR` | -20 | yes | Not a directory. |
| `EISDIR` | -21 | yes | Is a directory. |
| `EINVAL` | -22 | yes | Invalid argument. |
| `ENFILE` | -23 | no | Too many open files in system. |
| `EMFILE` | -24 | no | Too many open files. |
| `ENOTTY` | -25 | yes | Inappropriate ioctl for device. |
| `ETXTBSY` | -26 | yes | Text file busy. |
| `EFBIG` | -27 | no | File too large. |
| `ENOSPC` | -28 | no | No space left on device. |
| `ESPIPE` | -29 | no | Illegal seek. |
| `EROFS` | -30 | no | Read-only file system. |
| `EMLINK` | -31 | no | Too many links. |
| `EPIPE` | -32 | no | Broken pipe. |
| `EDOM` | -33 | no | Numerical argument out of domain. |
| `ERANGE` | -34 | yes | Numerical result out of range. |
| `EDEADLK` | -35 | no | Resource deadlock avoided. |
| `ENAMETOOLONG` | -36 | no | File name too long. |
| `ENOLCK` | -37 | no | No locks available. |
| `ENOSYS` | -38 | yes | Function not implemented. |
| `ENOTEMPTY` | -39 | yes | Directory not empty. |
| `ELOOP` | -40 | no | Too many levels of symbolic links. |
| `EWOULDBLOCK` | -11 | no | Alias of `EAGAIN`. |
| `EDEADLOCK` | -35 | no | Alias of `EDEADLK`. |
| `ENOMSG` | -42 | no | No message of desired type. |
| `EIDRM` | -43 | no | Identifier removed. |
| `ECHRNG` | -44 | no | Channel number out of range. |
| `EL2NSYNC` | -45 | no | Level 2 not synchronized. |
| `EL3HLT` | -46 | no | Level 3 halted. |
| `EL3RST` | -47 | no | Level 3 reset. |
| `ELNRNG` | -48 | no | Link number out of range. |
| `EUNATCH` | -49 | no | Protocol driver not attached. |
| `ENOCSI` | -50 | no | No CSI structure available. |
| `EL2HLT` | -51 | no | Level 2 halted. |
| `EBADE` | -52 | no | Invalid exchange. |
| `EBADR` | -53 | no | Invalid request descriptor. |
| `EXFULL` | -54 | no | Exchange full. |
| `ENOANO` | -55 | no | No anode. |
| `EBADRQC` | -56 | no | Invalid request code. |
| `EBADSLT` | -57 | no | Invalid slot. |
| `EBFONT` | -59 | no | Bad font file format. |
| `ENOSTR` | -60 | no | Device not a stream. |
| `ENODATA` | -61 | no | No data available. |
| `ETIME` | -62 | no | Timer expired. |
| `ENOSR` | -63 | no | Out of streams resources. |
| `ENONET` | -64 | no | Machine is not on the network. |
| `ENOPKG` | -65 | no | Package not installed. |
| `EREMOTE` | -66 | no | Object is remote. |
| `ENOLINK` | -67 | no | Link has been severed. |
| `EADV` | -68 | no | Advertise error. |
| `ESRMNT` | -69 | no | Srmount error. |
| `ECOMM` | -70 | no | Communication error on send. |
| `EPROTO` | -71 | no | Protocol error. |
| `EMULTIHOP` | -72 | no | Multihop attempted. |
| `EDOTDOT` | -73 | no | RFS specific error. |
| `EBADMSG` | -74 | no | Bad message. |
| `EOVERFLOW` | -75 | no | Value too large for defined data type. |
| `ENOTUNIQ` | -76 | no | Name not unique on network. |
| `EBADFD` | -77 | no | File descriptor in bad state. |
| `EREMCHG` | -78 | no | Remote address changed. |
| `ELIBACC` | -79 | no | Can not access a needed shared library. |
| `ELIBBAD` | -80 | no | Accessing a corrupted shared library. |
| `ELIBSCN` | -81 | no | .lib section in a.out corrupted. |
| `ELIBMAX` | -82 | no | Attempting to link in too many shared libraries. |
| `ELIBEXEC` | -83 | no | Cannot exec a shared library directly. |
| `EILSEQ` | -84 | no | Invalid or incomplete multibyte or wide character. |
| `ERESTART` | -85 | no | Interrupted system call should be restarted. |
| `ESTRPIPE` | -86 | no | Streams pipe error. |
| `EUSERS` | -87 | no | Too many users. |
| `ENOTSOCK` | -88 | no | Socket operation on non-socket. |
| `EDESTADDRREQ` | -89 | no | Destination address required. |
| `EMSGSIZE` | -90 | no | Message too long. |
| `EPROTOTYPE` | -91 | no | Protocol wrong type for socket. |
| `ENOPROTOOPT` | -92 | no | Protocol not available. |
| `EPROTONOSUPPORT` | -93 | no | Protocol not supported. |
| `ESOCKTNOSUPPORT` | -94 | no | Socket type not supported. |
| `EOPNOTSUPP` | -95 | no | Operation not supported. |
| `EPFNOSUPPORT` | -96 | no | Protocol family not supported. |
| `EAFNOSUPPORT` | -97 | no | Address family not supported by protocol. |
| `EADDRINUSE` | -98 | no | Address already in use. |
| `EADDRNOTAVAIL` | -99 | no | Cannot assign requested address. |
| `ENETDOWN` | -100 | no | Network is down. |
| `ENETUNREACH` | -101 | no | Network is unreachable. |
| `ENETRESET` | -102 | no | Network dropped connection on reset. |
| `ECONNABORTED` | -103 | no | Software caused connection abort. |
| `ECONNRESET` | -104 | no | Connection reset by peer. |
| `ENOBUFS` | -105 | no | No buffer space available. |
| `EISCONN` | -106 | no | Transport endpoint is already connected. |
| `ENOTCONN` | -107 | no | Transport endpoint is not connected. |
| `ESHUTDOWN` | -108 | no | Cannot send after transport endpoint shutdown. |
| `ETOOMANYREFS` | -109 | no | Too many references: cannot splice. |
| `ETIMEDOUT` | -110 | no | Connection timed out. |
| `ECONNREFUSED` | -111 | no | Connection refused. |
| `EHOSTDOWN` | -112 | no | Host is down. |
| `EHOSTUNREACH` | -113 | no | No route to host. |
| `EALREADY` | -114 | no | Operation already in progress. |
| `EINPROGRESS` | -115 | no | Operation now in progress. |
| `ESTALE` | -116 | no | Stale file handle. |
| `EUCLEAN` | -117 | no | Structure needs cleaning. |
| `ENOTNAM` | -118 | no | Not a XENIX named type file. |
| `ENAVAIL` | -119 | no | No XENIX semaphores available. |
| `EISNAM` | -120 | no | Is a named type file. |
| `EREMOTEIO` | -121 | no | Remote I/O error. |
| `EDQUOT` | -122 | no | Disk quota exceeded. |
| `ENOMEDIUM` | -123 | no | No medium found. |
| `EMEDIUMTYPE` | -124 | no | Wrong medium type. |
| `ECANCELED` | -125 | no | Operation canceled. |
| `ENOKEY` | -126 | no | Required key not available. |
| `EKEYEXPIRED` | -127 | no | Key has expired. |
| `EKEYREVOKED` | -128 | no | Key has been revoked. |
| `EKEYREJECTED` | -129 | no | Key was rejected by service. |
| `EOWNERDEAD` | -130 | no | Owner died. |
| `ENOTRECOVERABLE` | -131 | no | State not recoverable. |
| `ERFKILL` | -132 | no | Operation not possible due to RF-kill. |
| `EHWPOISON` | -133 | no | Memory page has hardware error. |

## Limits

- `abi/syscalls.toml` publishes 25 codes and leaves out 9 of the 24 microkernel constants: `EFBIG`, `ENOSPC`, `EBADMSG`, `EOPNOTSUPP`, `ENETDOWN`, `ETIMEDOUT`, `EALREADY`, `EINPROGRESS` and `ESTALE`. It also leaves out `ENODATA`, `EPROTO` and `EMSGSIZE`, which the time and crypto calls return. A toolchain that reads only that file will not have names for them.
- `abi/syscalls.toml` lists `errno_range` down to -4095, but no value the kernel defines is below -133.
- When no microkernel group handles a number, `route_tail` returns -1, which reads as `EPERM`; no published number reaches that path at this commit (`src/syscall/microkernel/dispatch/route.rs:52-68`).

## See also

- [The NONOS ABI](README.md)
- [Syscalls](syscalls.md)
- [Broker](broker.md)
- [IPC](ipc.md)
- [abi/syscalls.toml](../../abi/syscalls.toml)
