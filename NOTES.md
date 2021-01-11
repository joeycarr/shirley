# TODO
 * make better use of the Default trait
 * use *output = Struct::new(...) more and remove the copy() methods we don't
   need.

# Performance / Profiling

Baseline for 400x266 100 samples per pixel, single threaded: 2m45s or 165
seconds.

Ran multi-threaded 400x266 at 104 samples per pixel, on eight threads, for a
wall clock runtime of 24.4 seconds. May have incidentally improved runtime by
using static dispatch in place of the hittable list, because trait objects seem
to be inherently unable to move across threads, possibly?

Single threaded throughput ran at 64,485 samples per second. The multi-threaded
version ran at 453,508 samples per second. Approximately a 7x speed up. It's
about what we'd expect for a short run time.

First single threaded run at 1200x800 with 500 samples per pixel ran at 116 minutes. That equated to 68,925 samples per second. Ran the multi-threaded version in 13 minutes 54 seconds, or 575,540 samples per second. That ended up being a little better than an 8x speedup.

--

9 January 2021 - Updated "baseline" using Arc references and trait objects
rather than the static-dispatch version from the original baseline. 400x266 at
104 samples per pixel ran in 88 seconds with eight threads. This version also
implemented motion blur. Recording it here as a baseline to compare after
implementing the BVH tree.

10 January 2021 - Re-ran the baseline accidentally because the AABB hit test was
dead code in my first test. Confirmed it at 85 seconds for eight threads.
Eliminated the dead code error and obtained a result of 28 seconds. For the
final scene from Book 1, plus motion blur, the BVH provides a 3x speedup.
Presumably that could be even better in some circumstances.

I may have fouled the test by running the 28 second version without the power
adapter, or while the processor was warmer? I've seen results as low as 15
seconds, which could imply as much as a 5.7x speedup with the BVH tree. Hard to
say. The inconsistency could also be due to the non-deterministic construction
of the BVH tree and the random construction of the scene itself.
