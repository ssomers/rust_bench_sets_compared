# rust_bench_sets_compared
Comparison of some of the performance of BTreeSet and HashSet and the standard set in Go, in particular:

- Microbenchmark of "peek", a method to query an arbitrary element from a set.
  This method isn't really useful in itself, but it's the part that can be microbenchmarked.
- Benchmark program for draining a set with "pop", a method to split off an arbitrary element from a set.
  This is useful in some algorithms, though a brighter idea is to do that in a truely random or smart order.
