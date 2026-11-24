# Migration: the vfs guarded trees

Findings: NKA-2026-015 (low), NKA-2026-049 (low). Both are A1 findings: a hostile capsule holding FileSystem but not ForeignExec learns about the Linux personality's private files. Target design: capability-scoped namespaces, [07-target-object-model.md](../07-target-object-model.md).

## Current state

`userland/capsule_vfs` (6,666 lines) serves every file a capsule keeps, from one store. The store is three flat vectors: every file in the system, every open descriptor and the recent-files journal (`userland/capsule_vfs/src/store/fdtable/types.rs:102-106`), up to 8,192 files (`userland/capsule_vfs/src/store/fdtable/types.rs:20`). Paths are global; there is no per-client root.

Three gates decide whether a request is served:

1. **FileSystem.** The sender must be the kernel-internal client, which arrives as sender pid 0, or a pid the kernel says holds bit 6, asked on every request (`userland/capsule_vfs/src/server/fs_gate.rs:44-52`, `userland/capsule_vfs/src/server/fs_gate/rule.rs:27-29`).
2. **The guarded trees.** `/linux-home` and `/linux-private` (`userland/capsule_vfs/src/server/fs_gate/guard.rs:30`) are reached only by a sender holding ForeignExec, which only the personality holds. `reaches` reads the paths a request names (`userland/capsule_vfs/src/server/fs_gate/paths.rs:32-43`) and, if any is guarded, asks the kernel about ForeignExec; for `OP_COPY` and `OP_RMDIR` it also refuses any ancestor of a guarded tree (`userland/capsule_vfs/src/server/fs_gate/reach.rs:31-43`).
3. **Per-handler filters.** Listings drop guarded entries (`userland/capsule_vfs/src/server/fs_gate/guard.rs:53-65`); search drops guarded hits after collecting them (`userland/capsule_vfs/src/server/handlers/search.rs:46-49`).

The Linux personality builds its guests' view on top: a family's `/` is `/linux` (or `/linux-deb`, `/linux-pacman`), private directories map to `/linux-private/<id>` and homes to `/linux-home/<key>` (`userland/capsule_linux/src/linux/file/root.rs:36`, `userland/capsule_linux/src/linux/file/private/names.rs:30`, `userland/capsule_linux/src/linux/file/private/home.rs:29`). Private directories are capped at 16 MiB and 1,536 names because they live in the store every capsule shares (`userland/capsule_linux/src/linux/file/system/declared/sizes.rs:71-72`).

The caller identity is a pid. A request's payload starts with a caller pid; for a ring-3 sender it must equal the kernel-stamped sender pid, and for sender 0 it is taken on trust (`userland/capsule_vfs/src/server/handlers/util.rs:53-77`). The only kernel-internal client, `src/fs/vfs_capsule/client`, has no caller at `551bbd34`, so the trust path for pid 0 serves nothing today.

## The boundary failure

Isolation is a filter that every handler must remember, over a store any handler can walk in full.

- `OP_SEARCH` has no arm in `named`, so the gate never asks about it; the store scans every file, sorts, truncates to `max_hits`, and the handler removes guarded hits afterwards (`userland/capsule_vfs/src/store/fdtable/search.rs:63-84`, `userland/capsule_vfs/src/server/handlers/search.rs:46-49`). Whether the reply is empty reveals whether a guarded file matched: a content oracle (NKA-2026-015).
- `OP_DIRSTAT` is gated with `guarded` on the named path, which is false for `/`, and the store sums everything under the prefix (`userland/capsule_vfs/src/server/fs_gate/reach.rs:37-42`, `userland/capsule_vfs/src/store/fdtable/dirstat.rs:32-40`). Subtraction gives the guarded trees' counts and sizes (NKA-2026-049).
- `OP_USAGE` returns the file count and bytes of the whole store (`userland/capsule_vfs/src/store/fdtable/usage.rs:22-25`, served at `userland/capsule_vfs/src/server/handlers/usage.rs:25`). It names no path, so no path filter can apply, and usage minus the visible directories' `dirstat` gives the guarded trees' totals. The file manager and the terminal read it for their disk bars (`userland/capsule_file_manager/src/fm/sidebar_usage.rs`, `userland/capsule_terminal/src/rail/disk_query.rs`). The audit did not test it; it is the residual the in-place fixes cannot close.

Every new operation that walks the store is a new chance to forget. The class is "a shared namespace protected by a per-operation filter".

## Target design

**A client's namespace is the capability it holds.** vfs keeps one store but every request is interpreted relative to a namespace root, and the store's API cannot walk outside a root.

- **Namespaces.** A namespace is a root directory node plus rights (read, write, create, install). The Linux personality's family trees, each guest family's private directory and each app's home are separate namespaces. The user's files (what the file manager shows) are a namespace whose root does not contain `/linux-home` or `/linux-private`: those trees are siblings of it in the store, not children.
- **Store API.** Every store operation takes a `&Namespace` and starts its walk at the namespace's root node. There is no operation that iterates all files. `search`, `dirstat`, `usage`, `list`, `journal` and `copy` are written against a subtree by construction, so a handler that forgets a check can reach only the caller's own namespace.
- **Path resolution** clamps `..` at the namespace root, as the personality's resolver already does for guests (`userland/capsule_linux/src/linux/file/root.rs:36`).
- **Identity.** In the target, each namespace is a badged endpoint capability vfs mints; the badge selects the namespace. Before the object model exists, a namespace table in vfs maps a kernel-stamped sender pid to its namespaces, written only by init over a separate admin endpoint.
- **The private run directories** leave the shared store. They are cleared at the end of every run (`docs/userland/linux-personality.md` describes this), so they belong in memory the personality owns: a per-run ramfs instance, or the personality's own memory. That removes both the shared-store size caps and the guarded tree `/linux-private`.
- **The kernel-internal client** and the sender-0 trust path are deleted; nothing in the kernel needs to read files.
- **The Linux trees get their own store.** `/linux-home` and the family trees move to a second store partition with a capacity drawn from the volume once, at install or on first boot, and fixed thereafter. Every accounting operation (usage, free space, the 8,192-file limit) is per store, so the user store's figures never include a Linux file, and the Linux store's occupancy is visible only to its own namespaces. A fixed reservation inside one store cannot do this, because the number of families and per-app homes varies; a separate partition with a fixed size can.
- **Free space for ordinary capsules** is the user store's own: used bytes, used names and its capacity, where its capacity is the volume's capacity less the Linux partition's fixed size. That subtraction is a constant set at install, so it reveals nothing about what the Linux trees hold or when they change. The file manager's and terminal's disk bars show the user store; the personality reports its partition's figures to its guests (as `statfs`) from the Linux store.

## Interface specification

### Admin endpoint (init only)

```rust
/// Requests on vfs's admin endpoint. Before the object model, vfs accepts them
/// only from the sender pid registered as init; after, only on the admin
/// endpoint capability, which init alone holds.
enum AdminRequest {
    /// Create a namespace rooted at `root`, an absolute store path that must
    /// not be inside another namespace's root unless `nest` is set.
    CreateNamespace { root: [u8; 255], root_len: u8, nest: bool } , // -> NamespaceId
    /// Interim: bind a pid to a namespace with rights. Replaced by MintNamespaceCap.
    Bind { pid: u32, ns: NamespaceId, rights: NsRights },
    /// Target: mint a badged endpoint capability for one namespace.
    MintNamespaceCap { ns: NamespaceId, rights: NsRights }, // -> CapHandle, transferred in the reply
    Revoke { ns: NamespaceId },
}

type NamespaceId = u32;
bitflags! { struct NsRights: u16 { READ = 1; WRITE = 2; CREATE = 4; INSTALL = 8; } }
```

### Client protocol changes

- The caller-pid prefix in every payload (`split_caller`, `userland/capsule_vfs/src/server/handlers/util.rs:68-77`) is removed. The namespace comes from the sender (interim) or the badge (target).
- Paths stay absolute strings, now absolute within the namespace.
- A client bound to several namespaces (the personality: its family tree, the guest's home) selects one with a `u32` namespace index in the request header, which vfs checks against the sender's bindings.
- Errors: `EACCES` when the sender has no binding for the namespace or lacks the right; `ENOENT` for any path outside it, never a different error, so nothing outside a namespace is distinguishable from absence.

### Store API

```rust
pub struct Namespace { root: NodeId, rights: NsRights }

impl Store {
    pub fn open(&mut self, ns: &Namespace, path: &str, flags: u32, owner: u32) -> Result<Fd, Errno>;
    pub fn search(&self, ns: &Namespace, query: &str, flags: u32, max_hits: usize) -> Vec<Hit>;
    pub fn dirstat(&self, ns: &Namespace, path: &str, max_nodes: usize) -> DirStat;
    pub fn usage(&self, ns: &Namespace) -> Usage;
    // ... every operation takes &Namespace; no operation takes a bare global path
}
```

To make "cannot walk outside a root" structural, the flat `files: Vec<File>` becomes a tree of nodes with children lists, and a `Namespace` holds a `NodeId`. A walk from a node can only reach its descendants.

## Migration steps

1. **Fix the two findings in place.** This is done: a scope table classifies every operation as acting on a named path, a subtree, a handle or the whole store, and an operation with no class is refused; and guarded nodes are removed from a walk before any count, sort or cap.
   *Invariant*: a caller without ForeignExec learns nothing about guarded trees from any path or subtree operation. *Test*: the audit's harnesses for both findings, as host tests in `userland/fs_proofs`. The whole-store `OP_USAGE` stays open after this step and is closed by step 5.
2. **Delete the sender-0 trust path.** Remove `src/fs/vfs_capsule/client` and the `sender_pid == 0` branches (`userland/capsule_vfs/src/server/fs_gate/rule.rs:27-29`, `userland/capsule_vfs/src/server/handlers/util.rs:53-56`).
   *Invariant*: every request is attributed to a ring-3 sender. *Test*: a request stamped as sender 0 is refused.
3. **Move private run directories out of the store.** The personality keeps `/tmp`, `/dev/shm`, `/run` and the other private directories in a per-run ramfs instance or its own memory. `/linux-private` disappears from the store.
   *Invariant*: guests see the same private directories, with the same limits. *Test*: `capsule_linux_proofs` private-directory tests; the GTK mime-database build that needs 1,068 names.
4. **Introduce the node tree and namespaces in the store**, with one namespace, `/`, bound to every FileSystem holder. No behaviour changes.
   *Invariant*: every operation returns what it returned before. *Test*: the full `fs_proofs` suite.
5. **Split the stores and the namespaces.** The Linux trees move to their own store partition with a fixed capacity; `OP_USAGE` and every other accounting operation answer per store. Init creates the user namespace over the user store, one namespace per Linux family tree and per app home over the Linux store, and binds them: the file manager and other FileSystem holders to the user namespace, the personality to its family trees and homes. `guarded` and `touches` stay as a second check.
   *Invariant*: no client bound only to the user namespace can name, list, search, count or copy anything in a Linux tree, and the user store's usage figures do not change when a Linux file is written. *Test*: a test that writes a guarded file and checks that `OP_USAGE` for a user-namespace client is unchanged; the step-1 regression tests, now passing with the filters disabled in a test build, which shows the isolation is structural.
6. **Badged namespace capabilities.** When endpoints and capability transfer exist, vfs mints one badged capability per namespace; init hands them out; the admin endpoint's `Bind` and the kernel's `mk_cap_check` for FileSystem and ForeignExec are removed from vfs.
   *Invariant*: as step 5. *Test*: as step 5, plus a test that a capability for one namespace cannot be used for another.

## Does the in-place fix stay necessary?

Yes until step 5, and step 1's version is already in place. After step 5 the filters are redundant, and the `OP_USAGE` residual is closed by the separate store: a request from a client without a Linux namespace never reaches a Linux node. Keep them as a defence against a mistake in namespace assignment; they cost a string comparison.

## Risks

- **Store layout change.** The node tree changes `capsule_vfs`'s persistent layout through `mk_store_write`. Step 4 must convert the existing table of contents on first boot and keep reading the old one until a release has passed.
- **Cross-namespace operations.** Copying a file from the user namespace into a Linux home (an install that seeds a home) needs a client bound to both. Only the installer and the personality should be; the bindings are init's data and are reviewed with it.
- **The journal.** The recent-files journal is global today. It becomes per namespace, or it records only user-namespace files, which is what `guard.rs` already intends.
