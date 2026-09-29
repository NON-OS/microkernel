/*
 * Go's os, io/fs and path/filepath as a Go program uses them: a tree made
 * with MkdirAll and read back with ReadDir and WalkDir, CreateTemp, a copy
 * through io.Copy (which Go does with copy_file_range), Chmod, Chtimes,
 * Truncate, Rename, Symlink, Readlink and Lstat, Executable, Hostname
 * against uname, NumCPU against the CPUs /proc/cpuinfo lists, and
 * syscall.Flock between two opens. Every part prints; nothing printed
 * depends on the machine, so the host prints the same lines.
 */
package main

import (
	"fmt"
	"os"
	"strings"
)

var parts, failed int

var bad []string

func check(part string, good bool, what string, a ...any) bool {
	if !good {
		fmt.Printf("[GO] goos %s FAIL: %s %v\n", part, what, a)
		failed++
		bad = append(bad, part)
	}
	return good
}

func done(part, detail string) {
	parts++
	fmt.Printf("[GO] goos %s ok: %s\n", part, detail)
}

const root = "/tmp/goos"

func main() {
	os.RemoveAll(root)
	tree()
	temp()
	files()
	system()
	locking()
	os.RemoveAll(root)
	if failed > 0 {
		fmt.Printf("[GO] goos FAIL: %d of %d parts: %s\n", failed, parts+failed, strings.Join(bad, " "))
		os.Exit(1)
	}
	fmt.Printf("[GO] goos PASS: %d parts\n", parts)
}
