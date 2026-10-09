// A static Go binary, the cheapest tier-1 guest: no cgo, no dynamic loader.
// It exercises the runtime coming up (threads, memory, signals, GC) and
// prints a line the boot log can check.
package main

import (
	"fmt"
	"os"
	"runtime"
)

func main() {
	// Touch the heap enough to force at least one GC cycle.
	acc := 0
	for i := 0; i < 200000; i++ {
		s := make([]byte, 32)
		acc += len(s)
	}
	runtime.GC()
	fmt.Printf("[GO] hello: %s GOMAXPROCS=%d NumCPU=%d touched=%d\n",
		runtime.Version(), runtime.GOMAXPROCS(0), runtime.NumCPU(), acc)
	fmt.Println("[GO] hello PASS")
	os.Exit(0)
}
