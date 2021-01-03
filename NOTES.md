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
