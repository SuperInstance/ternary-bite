# ternary-bite

Destructive signal transformations for ternary data. Crush, quantize, downsample, bit-rotate, wavefold.

Where `ternary-warp` gives you clean, lossless transformations, `ternary-bite` gives you the *destructive* ones. Bit crushing that reduces temporal resolution. Quantization that snaps to fewer levels. Downsampling that averages away detail. Wavefolding that mirrors values past a threshold. Bit rotation that cycles through ternary space.

These are the operations you reach for when you *want* to lose information—when the signal has too much detail and you need the coarse shape underneath.

## Why this exists

Not every signal needs full fidelity. In fact, most interesting patterns in ternary data emerge *after* you crush away the noise. A raw sensor stream at full resolution is harder to classify than the same stream after bit crushing at rate 4. The coarse version reveals the structure that fine detail obscures.

This crate is the audio engineer's toolkit applied to ternary signals: bit crushers, downsamplers, wavefolders. If you've ever used a guitar pedal to make a clean signal sound dirty, you understand the aesthetic.

## The key insight

Destructive transformations are *feature extraction*. When you crush a signal from rate 1 to rate 4, you're not losing information—you're keeping the *structural* information (what value held for each block) and discarding the *within-block* variation. That's exactly what a feature extractor does.

Bit rotation is the most interesting operation here. It treats {-1, 0, +1} as a cyclic group (Z₃) and rotates each value through the group:

```
shift=0:  [-1, 0, 1] → [-1, 0, 1]   (identity)
shift=1:  [-1, 0, 1] → [ 0, 1,-1]   (rotate forward)
shift=-1: [-1, 0, 1] → [ 1,-1, 0]   (rotate backward)
```

This is ternary's analog of bit rotation in binary. It preserves the *relative* structure of the signal while shifting all values uniformly.

## Quick start

```rust
use ternary_bite::*;

// Bit crush: hold each value for `rate` samples
let signal = vec![10, 20, 30, 40, 50, 60, 70, 80];
let crushed = crush(&signal, 4);
assert_eq!(crushed, vec![10, 10, 10, 10, 50, 50, 50, 50]);

// Downsample: average blocks of `factor` samples
let data = vec![10, 20, 30, 40];
let down = downsample(&data, 2);
assert_eq!(down, vec![15, 35]);  // (10+20)/2, (30+40)/2

// Bit rotate: cycle through ternary group
let ternary = vec![-1, 0, 1];
let rotated = bit_rotate(&ternary, 1);
assert_eq!(rotated, vec![0, 1, -1]);

// Wavefold: mirror values past threshold
let folded = fold(&[80], 50);
assert_eq!(folded, vec![20]);  // 2*50 - 80 = 20
```

## API reference

### `crush(signal, rate) → Vec<i8>`

Bit crushing. Samples the first value of each `rate`-sized block and holds it for the block's duration. Rate 0 passes through unchanged.

```rust
crush(&[1, 2, 3, 4], 1)   // → [1, 2, 3, 4]  (identity)
crush(&[1, 2, 3, 4], 2)   // → [1, 1, 3, 3]   (hold first of each pair)
crush(&[5, 6, 7, 8], 0)   // → [5, 6, 7, 8]   (passthrough)
```

### `quantize(signal, levels) → Vec<i8>`

Snap values to `levels` evenly-spaced steps centered at 0. `levels=1` maps everything to 0.

```rust
quantize(&[10, 20, 30], 1)   // → [0, 0, 0]
quantize(&[0, 64, -64], 3)   // → quantized to 3 levels
```

### `downsample(signal, factor) → Vec<i8>`

Reduce temporal resolution by averaging blocks of `factor` samples. Factor ≤ 1 passes through unchanged. Handles uneven final blocks.

```rust
downsample(&[10, 20, 30, 40], 2)   // → [15, 35]
downsample(&[10, 20, 30], 2)        // → [15, 30]  (uneven last block)
```

### `bit_rotate(signal, shift) → Vec<i8>`

Rotate each value through ternary space {-1, 0, +1} by `shift` positions. This is modular arithmetic on the group Z₃.

```rust
bit_rotate(&[-1, 0, 1], 0)    // → [-1, 0, 1]   (identity)
bit_rotate(&[-1, 0, 1], 1)    // → [0, 1, -1]    (forward rotation)
bit_rotate(&[-1, 0, 1], -1)   // → [1, -1, 0]    (backward rotation)
bit_rotate(&[-1, 0, 1], 3)    // → [-1, 0, 1]    (full cycle = identity)
```

### `fold(signal, threshold) → Vec<i8>`

Wavefolding distortion. Values within `[-threshold, threshold]` pass through unchanged. Values exceeding the threshold are reflected back, creating harmonic distortion.

```rust
fold(&[5, -5, 0], 50)     // → [5, -5, 0]   (all within threshold)
fold(&[80], 50)            // → [20]          (2*50 - 80 = 20)
fold(&[-80], 50)           // → [-20]         (-2*50 - (-80) = -20)
fold(&[10, -10, 0], 0)    // → [0, 0, 0]     (zero threshold = silence)
```

## Composing with ternary-warp

`ternary-bite` and `ternary-warp` are designed to compose:

```rust
use ternary_bite::*;
// use ternary_warp::*;  // if you need both

// Coarse-grain a signal, then rotate it
let signal = vec![1, -1, 0, 1, 0, -1, 1, 0];
let crushed = crush(&signal, 2);           // [1, 1, 0, 0, 1, 1, 1, 1] (approximately)
let rotated = bit_rotate(&crushed, 1);     // shift all values through Z₃
```

## Architecture

All functions are pure: `&[i8]` in, `Vec<i8>` out. No state, no side effects, no global mutable anything. Each function allocates exactly one `Vec` for the output.

The crate works on full `i8` range, not just {-1, 0, +1}. This is intentional—`crush`, `downsample`, `quantize`, and `fold` all produce meaningful results on any `i8` signal. `bit_rotate` specifically operates in ternary space but handles arbitrary inputs by mapping them through the Z₃ group.

## Real-world example: Signal coarse-graining

```rust
use ternary_bite::*;

// A high-resolution ternary signal from a multi-agent simulation
let raw_signal: Vec<i8> = (0..100)
    .map(|i| ((i * 7 + 3) % 5 - 2) as i8)
    .collect();

// Step 1: Crush to reveal temporal structure
let crushed = crush(&raw_signal, 10);

// Step 2: Downsample to reduce dimensionality
let downsampled = downsample(&crushed, 2);

// Step 3: Quantize to ternary (if signal went outside {-1, 0, 1})
let quantized: Vec<i8> = downsampled.iter()
    .map(|&v| v.clamp(-1, 1))
    .collect();

// Step 4: Rotate to shift perspective (e.g., -1 → 0, 0 → 1, 1 → -1)
let shifted = bit_rotate(&quantized, 1);

println!("Raw: {} samples", raw_signal.len());      // 100
println!("After pipeline: {} samples", shifted.len()); // ~5
```

## Ecosystem connections

- **ternary-warp** — non-destructive transformations (clamp, smooth, differentiate). The yang to this crate's yin.
- **ternary-gauge** — measure your signal before and after destructive transforms to verify you kept what matters
- **ternary-complexity** — after coarse-graining, measure the Kolmogorov complexity to see if the signal actually simplified

## Stats

| Metric | Value |
|--------|-------|
| Tests | 16 |
| Public functions | 5 |
| Lines of code | ~210 |
| License | MIT |
| Unsafe | 0 |

## Installation

```toml
[dependencies]
ternary-bite = "0.1.0"
```

## License

MIT
