# rust_bench_sets_compared
Comparison of some of the performance of BTreeSet and HashSet and the standard set in Go, in particular:

- Microbenchmark of "peek", a method to query an arbitrary element from a set.
  This method isn't really useful in itself, but it's the part of "pop" that can be micro-benchmarked.

      cargo test
      cargo bench
      go test -bench=.

  Conclusion is that peek is pretty fast in Rust's standard library (compared to Go) but it's relatively bad in BTreeSet compared to hash sets.

- Benchmark program for draining a set with "pop", a method to split off an arbitrary element from a set.

  This is very different from the standard drain method because the (shrinking) set remains valid and usable after each pop. You can even remove more elements, or add others, in each step. It helps in some relatively simple algorithms (e.g. the basic form, without vertex ordering, of [the Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm)).
  
  Since this "pop" does not prescribe any particular order (like "smallest" element first), its implementation can choose an element that can be removed efficiently, in the current state of the data structure chosen to represent the set. That speeds up all simple algorithms using "pop", though it's probably better to let them visit elements in a random or a smart order (like the non-basic forms of Bron-Kerbosch do). To compare times:
 
      cargo run --release
      go run sets_compared.go

  Conclusion is that Go beats Rust's HashSet, but Rust's BTreeSet outshines both.
[![Time spent popping all elements](https://plot.ly/~stein.somers/288.png?share_key=L1Ip7GHTXN5mkrNimTdFVe "View interactively")](https://plot.ly/~stein.somers/288/?share_key=L1Ip7GHTXN5mkrNimTdFVe)
