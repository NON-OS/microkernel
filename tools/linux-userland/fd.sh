# fd 10.5.0 (the fd-find crate), the file finder. Built without its one
# default feature, which only adds a --gen-completions flag for shells NONOS
# does not run, and without use-jemalloc, a C allocator it would link in place
# of musl's own.
#   OUT/fd
. "$root/tools/linux-userland/lib/crate.sh"
crate_build fd-find fd-find-10.5.0 fd --no-default-features
