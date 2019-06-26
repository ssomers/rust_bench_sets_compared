// To run me: go test -bench=.
package main

import (
	"testing"
)

func PickArbitrary(set *Set) Key {
	for v, _ := range *set {
		return v
	}
	panic("attempt to pick from empty set")
}

func BenchPeek(b *testing.B, size Key) {
	set := make(Set)
	for i := Key(0); i < size; i++ {
		set[i] = struct{}{}
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		PickArbitrary(&set)
	}
}

func BenchmarkPeek1(b *testing.B) {
	BenchPeek(b, 1e0)
}

func BenchmarkPeek100(b *testing.B) {
	BenchPeek(b, 1e2)
}

func BenchmarkPeek10k(b *testing.B) {
	BenchPeek(b, 1e4)
}

func BenchmarkPeek1kk(b *testing.B) {
	BenchPeek(b, 1e6)
}
