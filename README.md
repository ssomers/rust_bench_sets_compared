# rust_bench_sets_compared
Comparison of some of the performance of BTreeSet and HashSet and the standard set in Go, in particular:

- Microbenchmark of "peek", a method to query an arbitrary element from a set.
  This method isn't really useful in itself, but it's the part of "pop" that can be micro-benchmarked.

      cargo test
      cargo bench
      go test -bench=.

  Conclusion (for 32 bit elements on 64 bit architecture) is that peek is pretty fast in Rust's standard library (compared to Go) but it's relatively bad in BTreeSet compared to hash sets.

  Rust `BTreeSet`:

      test peek_btreeset_1         ... bench:           8 ns/iter (+/- 0)
      test peek_btreeset_100       ... bench:          10 ns/iter (+/- 0)
      test peek_btreeset_10k       ... bench:          11 ns/iter (+/- 0)
      test peek_btreeset_1kk       ... bench:          12 ns/iter (+/- 0)

  Rust `HashSet`:
  
      test peek_hashset_1          ... bench:           1 ns/iter (+/- 0)
      test peek_hashset_100        ... bench:           1 ns/iter (+/- 0)
      test peek_hashset_10k        ... bench:           1 ns/iter (+/- 0)
      test peek_hashset_1kk        ... bench:           1 ns/iter (+/- 0)

  Rust `fnv::FnvHashSet`:
  
      test peek_fnvhashtreeset_1   ... bench:           1 ns/iter (+/- 0)
      test peek_fnvhashtreeset_100 ... bench:           1 ns/iter (+/- 0)
      test peek_fnvhashtreeset_10k ... bench:           1 ns/iter (+/- 0)
      test peek_fnvhashtreeset_1kk ... bench:           1 ns/iter (+/- 0)
      
  Rust `hashbrown::HashSet`:
      
      test peek_hashbrownset_1     ... bench:           1 ns/iter (+/- 0)
      test peek_hashbrownset_100   ... bench:           1 ns/iter (+/- 0)
      test peek_hashbrownset_10k   ... bench:           1 ns/iter (+/- 0)
      test peek_hashbrownset_1kk   ... bench:           1 ns/iter (+/- 0)

  Go `map[uint32]struct{}`:

      BenchmarkPeek1-6        42987794                28.4 ns/op
      BenchmarkPeek100-6      50083262                23.0 ns/op
      BenchmarkPeek10k-6      44564107                27.7 ns/op
      BenchmarkPeek1kk-6      19406798                62.1 ns/op

- Benchmark program for draining a set with "pop", a method to split off an arbitrary element from a set.

  This is very different from the standard drain method because the (shrinking) set remains valid and usable after each pop. You can even remove more elements, or add others, in each step. It helps in some relatively simple algorithms (e.g. the basic form, without vertex ordering, of [the Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm)).
  
  Since this "pop" does not prescribe any particular order (like "smallest" element first), its implementation can choose an element that can be removed efficiently, in the current state of the data structure chosen to represent the set. That speeds up all simple algorithms using "pop", but it's easy to unknowingly rely on the arbitrary choice, it's harder to debug, and if the goal is best performance, it's probably better to let algorithms visit elements in a random or a smart order (like the non-basic forms of Bron-Kerbosch do).
  
  For an ordered set like BTreeSet, we can simply use a "pop_first" that has a deterministic outcome and should also work efficiently, [if it were stabilized](https://github.com/rust-lang/rust/pull/65637).
  
  To compare times:
 
      cargo run --release
      go run sets_compared.go

  Conclusion is that Go beats Rust's HashSet, but Rust's BTreeSet outshines both.
[![Time spent popping all elements](https://plot.ly/~stein.somers/290.png?share_key=b7J3hVET9wszKlyaW9OFMQ)](https://plot.ly/~stein.somers/290/?share_key=b7J3hVET9wszKlyaW9OFMQ)
