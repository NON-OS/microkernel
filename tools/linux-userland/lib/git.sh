# Sources pinned by commit rather than by tarball: a sources.txt line whose
# URL is git+REPOSITORY@COMMIT and whose hash is the tree's Nix hash. A recipe
# sources this file and calls fetch_git NAME, which prints a directory holding
# that commit's tree. The flake fetches the tree itself, checks it against
# the hash and hands it over in NONOS_SRC_<NAME>; otherwise the commit is
# fetched with git into the source cache, and refused unless HEAD is the pin.
fetch_git() {
	set -- "$1" $(awk -v n="$1" '$1 == n { print $2 }' "$root/tools/nix/sources.txt")
	[ $# -eq 2 ] || { echo "nonos-linux-userland-build: no pin for $1" >&3; exit 1; }
	repo=${2#git+}
	rev=${repo##*@}
	repo=${repo%@*}
	dir="$cache/$1-$rev"
	given=$(eval "echo \"\${NONOS_SRC_$1:-}\"")
	if [ -n "$given" ]; then
		rm -rf "$dir" && cp -R "$given" "$dir" && chmod -R u+w "$dir"
	else
		if [ ! -f "$dir/.rev" ]; then
			rm -rf "$dir" && git init -q "$dir"
			git -C "$dir" fetch -q --depth 1 "$repo" "$rev"
			git -C "$dir" checkout -q FETCH_HEAD
			echo "$rev" >"$dir/.rev"
		fi
		if [ "$(git -C "$dir" rev-parse HEAD)" != "$rev" ]; then
			echo "nonos-linux-userland-build: $1 is not $rev" >&3
			exit 1
		fi
	fi
	echo "$dir"
}
