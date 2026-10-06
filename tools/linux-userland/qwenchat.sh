# The Qwen conversation, on llama.cpp at its pinned commit, one build per
# instruction set.
#   OUT/qwenchat (x86-64-v3), OUT/qwenchat-x86_64_v2, OUT/qwenchat-x86_64
# llama.cpp at the commit the guest lane proves against the host.
rev=$(awk '$1 == "llama.cpp" { sub(/.*@/, "", $2); print $2 }' "$root/tools/nix/sources.txt")
llama="$cache/llama.cpp-$rev"
# The flake hands over the tree it fetched and checked by hash.
if [ -n "${NONOS_LLAMA_SRC:-}" ]; then
	rm -rf "$llama" && cp -R "$NONOS_LLAMA_SRC" "$llama" && chmod -R u+w "$llama"
elif [ ! -f "$llama/.rev" ]; then
	rm -rf "$llama" && git init -q "$llama"
	git -C "$llama" fetch -q --depth 1 https://github.com/ggml-org/llama.cpp "$rev"
	git -C "$llama" checkout -q FETCH_HEAD
	echo "$rev" >"$llama/.rev"
fi
if [ -z "${NONOS_LLAMA_SRC:-}" ] && [ "$(git -C "$llama" rev-parse HEAD)" != "$rev" ]; then
	echo "nonos-linux-userland-build: llama.cpp is not $rev" >&3
	exit 1
fi
cmake=$("$root/tools/nonos-cmake")
# cmake runs with nothing of the host's: ggml and llama.cpp ask git for a
# commit to bake into the library (GGML_COMMIT), and a copied tree with no
# .git of its own sits inside this repository, so git answered with whatever
# commit the NONOS checkout was on, or nothing where git was absent. With no
# repository to find, both read "unknown" on every host. CFLAGS and LDFLAGS,
# which the driver sets and a devshell may export, would reach cmake's own
# probes as flags for every test compile.
qcmake() {
	env -u CFLAGS -u CXXFLAGS -u CPPFLAGS -u LDFLAGS \
		GIT_DIR=/nonexistent/no-git GIT_CEILING_DIRECTORIES=/ "$cmake" "$@"
}
# cmake takes a compiler as one program, so each tool gets a wrapper.
mkdir -p "$work/zig"
for t in cc c++; do
	printf '#!/bin/sh\nexec "%s" %s -target x86_64-linux-musl "$@"\n' "$zig" "$t" >"$work/zig/$t"
done
for t in ar ranlib; do printf '#!/bin/sh\nexec "%s" %s "$@"\n' "$zig" "$t" >"$work/zig/$t"; done
chmod +x "$work/zig/"*
cpp="$root/userland/linux_guests/cpp"
set -- "$cpp"/qwenchat*.cpp "$cpp"/qwenwl_*.cpp "$cpp"/qwenui_*.cpp \
	"$cpp/qwenpool.cpp" "$cpp/qwenmem.cpp" "$cpp/qwenmem_file.cpp"
for level in x86_64_v3 x86_64_v2 x86_64; do
	b="$work/llama-$level"
	# A cross build: without the system named, cmake on macOS adds its
	# own -arch and -isysroot, which no Linux target takes.
	qcmake -S "$llama" -B "$b" -DCMAKE_BUILD_TYPE=Release \
		-DCMAKE_SYSTEM_NAME=Linux -DCMAKE_SYSTEM_PROCESSOR=x86_64 \
		-DCMAKE_C_COMPILER="$work/zig/cc" -DCMAKE_CXX_COMPILER="$work/zig/c++" \
		-DCMAKE_AR="$work/zig/ar" -DCMAKE_RANLIB="$work/zig/ranlib" \
		-DCMAKE_C_FLAGS="-mcpu=$level -g0 -ffile-prefix-map=$cache=/src" \
		-DCMAKE_CXX_FLAGS="-mcpu=$level -g0 -ffile-prefix-map=$cache=/src" \
		-DBUILD_SHARED_LIBS=OFF -DGGML_STATIC=ON -DGGML_NATIVE=OFF -DGGML_OPENMP=OFF \
		-DGGML_CCACHE=OFF -DGGML_BACKEND_DL=OFF -DLLAMA_OPENSSL=OFF -DLLAMA_BUILD_COMMON=OFF \
		-DLLAMA_BUILD_TESTS=OFF -DLLAMA_BUILD_TOOLS=OFF -DLLAMA_BUILD_EXAMPLES=OFF \
		-DLLAMA_BUILD_SERVER=OFF -DLLAMA_BUILD_APP=OFF >/dev/null
	qcmake --build "$b" -j8 --target llama >/dev/null
	# The page-safe string functions every Qwen guest links (qwenlibc.c).
	"$work/zig/cc" -c -O2 -g0 -mcpu="$level" -std=c11 -fno-builtin \
		-ffile-prefix-map="$root=/nonos" "$cpp/qwenlibc.c" -o "$work/qwenlibc-$level.o"
	case $level in x86_64_v3) name=qwenchat ;; *) name=qwenchat-$level ;; esac
	"$work/zig/c++" -static -no-pie -s -O2 -g0 -mcpu="$level" -std=c++17 \
		-ffile-prefix-map="$root=/nonos" -ffile-prefix-map="$cache=/src" \
		-I"$llama/include" -I"$llama/ggml/include" "$@" "$work/qwenlibc-$level.o" \
		-o "$out/$name" "$b/src/libllama.a" "$b/ggml/src/libggml.a" \
		"$b/ggml/src/libggml-cpu.a" "$b/ggml/src/libggml-base.a" -lpthread
done
