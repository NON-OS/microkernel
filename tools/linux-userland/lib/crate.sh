# Rust programs published on crates.io, built from the .crate file at the
# checksum the crates.io index publishes for that version, against the
# Cargo.lock the crate was published with. A copy of that lock is kept in
# tools/nix/locks, from which the flake vendors every dependency by its own
# checksum (tools/nix/vendor.nix), and the build stops if the two differ.
# Outside the flake cargo fetches the same locked crates.
#
# crate_build PIN DIR BIN [CARGO ARGS...]: unpack the crate pinned as PIN,
# which unpacks to DIR, and build its BIN for x86_64-unknown-linux-musl,
# static and non-PIE, into OUT/BIN.
crate_build() {
	pin=$1 dir=$2 bin=$3
	shift 3
	tar -xzf "$(fetch "$pin")" -C "$work"
	c="$work/$dir"
	if ! cmp -s "$c/Cargo.lock" "$root/tools/nix/locks/$dir.Cargo.lock"; then
		echo "nonos-linux-userland-build: $dir's Cargo.lock is not tools/nix/locks/$dir.Cargo.lock" >&3
		exit 1
	fi
	# A crate with C inside (ripgrep's jemalloc on 64-bit musl) compiles it
	# with the pinned zig, through cargo's per-target CC and AR. The cc crate
	# names the target in Rust's spelling, which zig does not take; the
	# wrapper names it in zig's.
	mkdir -p "$work/zig"
	cat >"$work/zig/cc" <<-CC
	#!/bin/sh
	for a in "\$@"; do
		shift
		case "\$a" in --target=*) ;; *) set -- "\$@" "\$a" ;; esac
	done
	exec "$zig" cc -target x86_64-linux-musl "\$@"
	CC
	printf '#!/bin/sh\nexec "%s" ar "$@"\n' "$zig" >"$work/zig/ar"
	chmod +x "$work/zig/cc" "$work/zig/ar"
	# rustc names its own sources and the crates' paths in panic messages, so
	# each is mapped to a fixed name (the flake's NONOS_REMAP does the same for
	# its vendor directory). zig links, with its own musl and start files in
	# place of the copy Rust carries, so every program in the store has the
	# one libc.
	sysroot=$(rustc --print sysroot)
	remap="--remap-path-prefix=$work=/build --remap-path-prefix=$sysroot=/rust"
	remap="$remap --remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=/cargo"
	(cd "$c" &&
		CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="$work/zig/cc" \
		CARGO_TARGET_DIR="$work/target-$bin" \
		CC_x86_64_unknown_linux_musl="$work/zig/cc" AR_x86_64_unknown_linux_musl="$work/zig/ar" \
		CFLAGS_x86_64_unknown_linux_musl="-O2 -g0 -fno-pie -ffile-prefix-map=$work=/build" \
		RUSTFLAGS="-C target-feature=+crt-static -C relocation-model=static -C strip=symbols -C codegen-units=1 -C link-self-contained=no $remap" \
		cargo build --locked --release --target x86_64-unknown-linux-musl --bin "$bin" "$@" >&2)
	cp "$work/target-$bin/x86_64-unknown-linux-musl/release/$bin" "$out/$bin"
}
