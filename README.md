# easy_rand

Small Rust wrapper library for calling POSIX's srand() and rand() functions.

Provides:
 - `c_srand(seed: u32)` : sets initial seed value by calling `srand()`
 - `c_rand()` : returns a pseudo-random number by calling `rand()`
 - `c_rand_range(min: i32, max: i32)` : returns a pseudo-random number in the range `[min, max[` via the following algorithm: ```rand() % (max - min) + min```

