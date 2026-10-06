# ripgrep 15.2.0, the recursive grep, with its default features (Rust's own
# regex engine; no PCRE2).
#   OUT/rg
. "$root/tools/linux-userland/lib/crate.sh"
crate_build ripgrep ripgrep-15.2.0 rg
