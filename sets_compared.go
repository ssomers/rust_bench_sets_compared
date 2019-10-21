// To run me: go run sets_compared.go
package main

import (
	"fmt"
	"math"
	"time"
)

type Key uint32
type Set map[Key]struct{}

type SampleStatistics struct {
	samples      int
	sum          float64
	sumOfSquares float64
}

func (s *SampleStatistics) Put(v float64) {
	s.samples++
	s.sum += v
	s.sumOfSquares += v * v
}

func (s *SampleStatistics) Mean() float64 {
	return s.sum / float64(s.samples)
}

func (s *SampleStatistics) Variance() float64 {
	n := float64(s.samples)
	numerator := s.sumOfSquares - s.sum*s.sum/n
	if numerator < 0 {
		return 0
	}
	return numerator / (n - 1)
}

func (s *SampleStatistics) Deviation() float64 {
	return math.Sqrt(s.Variance())
}

func PopArbitrary(set *Set) Key {
	for v, _ := range *set {
		delete(*set, v)
		return v
	}
	panic("attempt to pop from empty set")
}

func pop_set(N Key) float64 {
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
	return float64(ns) / 1e9
}

func main() {
	seconds_by_size := make(map[Key]*SampleStatistics)
	iterations := 9
	for I := 0; I <= iterations; I++ {
		if I < iterations {
			fmt.Printf("%d", iterations-I)
		} else {
			fmt.Printf(" done!\n     size seconds\n")
		}
		for N := Key(20e3); N < 700e3; {
			if I == 0 {
				seconds_by_size[N] = &SampleStatistics{}
			}
			stats := seconds_by_size[N]
			if I < iterations {
				seconds := pop_set(N)
				stats.Put(seconds)
			} else {
				mean := stats.Mean()
				dev := stats.Deviation() / mean
				fmt.Printf("%9d %.3f ±%.0f%%\n", N, mean, dev*100.0)
			}
			if N < 100e3 {
				N += 20e3
			} else {
				N += 100e3
			}
		}
	}
}
