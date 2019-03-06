// To run me: go run sets_compared.go
package main

import (
	"fmt"
	"time"
)

type Key uint32
type Set map[Key]struct{}

func PopArbitrary(set *Set) Key {
	for v, _ := range *set {
		delete(*set, v)
		return v
	}
	panic("attempt to pop from empty set")
}

func pop_set(N Key) {
	set := make(Set)
	for i := Key(0); i < N; i++ {
		set[i] = struct{}{}
	}

	var total uint64
	begin := time.Now()
	for len(set) > 0 {
		v := PopArbitrary(&set)
		total += uint64(v)
	}
	ns := time.Since(begin).Nanoseconds()
	if total != uint64(N)*uint64(N-1)/2 {
		panic(fmt.Sprintf("Got %d, expected %d", total, N*(N-1)/2))
	}
	fmt.Printf("Shrinking set size=%4dk: %4.3fs\n", N/1000, float64(ns)/1e9)
}

func main() {
	pop_set(100000)
	pop_set(200000)
	pop_set(300000)
	pop_set(400000)
	pop_set(1000000)
}
