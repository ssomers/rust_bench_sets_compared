# rust_bench_sets_compared
Comparison of some of the performance of BTreeSet and HashSet and the standard set in Go, in particular:

- Microbenchmark of "peek", a method to query an arbitrary element from a set.
  This method isn't really useful in itself, but it's the part that can be microbenchmarked.

      cargo test
      cargo bench
      go test -bench=.

  Conclusion is that peek is pretty fast in Rust's standard library (compared to Go) but it's relatively bad in BTreeSet compared to hash sets.

- Benchmark program for draining a set with "pop", a method to split off an arbitrary element from a set.
  This is different from the standard drain method because the (shrinking) set remains valid and usable after each pop.
  The arbitrary choice allows "pop" to be implemented more efficiently for the given data structure then alternative choice, like random, smallest...
  If it is efficiently implemented, it can boost some algorithms (e.g. the basic form (without vertex ordering) of [the Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm)). Though it's probably a brighter idea to implement a truely random or a smart order.

      cargo run --release
      go run sets_compared.go

