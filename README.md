# ternary-bite

Destructive signal transformations for ternary data. Crush, quantize, downsample, rotate, wavefold.

## The Problem

Not every signal needs full fidelity. A raw sensor stream at full resolution is harder to classify than the same stream after coarse-graining — the fine detail obscures the structure underneath. In audio, you reach for a bit crusher. In image processing, you downsample. In signal processing, you apply a low-pass filter.

For ternary signals {-1, 0, +1}, the equivalent operations don't exist in standard libraries. `crush()` isn't downsampling — it's *sample-and-hold*. `bit_rotate()` isn't bit shifting — it's cyclic permutation through the group Z₃. `fold()` isn't clipping — it's wavefolding, a mirror reflection that creates harmonic distortion. These are operations that reduce information deliberately, extracting coarse structure by destroying fine detail.

## The Insight

Destructive transformations are feature extraction. When you crush a signal from rate 1 to rate 4, you're keeping structural information (what value held for each block) and discarding within-block variation. When you downsample by factor 2, you're replacing pairs with their mean. When you bit-rotate, you're applying a group action that preserves relative structure while shifting all values uniformly.

The most interesting operation is `bit_rotate`: it treats {-1, 0, +1} as the cyclic group Z₃ and rotates each value through it:

```
shift=+1:  -1 → 0,  0 → +1,  +1 → -1    (forward rotation)
shift=-1:  -1 → +1, 0 → -1,  +1 → 0     (backward rotation)
shift=0:   identity
shift=3:   identity (full cycle)
```

This is ternary's analog of bit rotation in binary. It's a symmetry operation: it preserves the *relative* distances between elements while shifting all values. Every ternary signal has two rotated versions that are structurally identical but numerically distinct — useful for data augmentation or perspective-shifting in ternary networks.

Wavefolding is the second insight. In analog synthesizers, a wavefolder creates harmonics by reflecting a signal past a threshold: values within `[-t, t]` pass through, values beyond are mirrored back (`2t - v`). Applied to ternary signals, this creates controlled distortion — the output is still bounded, but the distribution of values changes. With threshold 1 and input {−1, 0, +1}, nothing changes. With threshold 0, everything folds to zero. The space in between is where it gets interesting.

## How It Works

Five pure functions. `&[i8]` in, `Vec<i8>` out. No state, no side effects.

### crush(signal, rate)

Sample-and-hold. Takes the first value of each `rate`-sized block and holds it for the block's duration. Rate 0 passes through unchanged.

```
input:  [10, 20, 30, 40, 50, 60, 70, 80]
rate 4: [10, 10, 10, 10, 50, 50, 50, 50]
         ^^^^ hold 10     ^^^^ hold 50
```

This is bit crushing from audio: reduce temporal resolution by holding values constant. The output length equals the input length — temporal structure is preserved, but within-block variation is destroyed.

### downsample(signal, factor)

Block averaging. Replaces each block of `factor` samples with their integer mean. Output length = `ceil(len / factor)`.

```
input:   [10, 20, 30, 40]
factor 2: [15, 35]    // (10+20)/2, (30+40)/2
```

Uneven final blocks are handled naturally: `[10, 20, 30]` with factor 2 → `[15, 30]` (the last block is just one element).

### bit_rotate(signal, shift)

Cyclic permutation through Z₃. Maps each value `v` to `((v+1+shift) mod 3) - 1`:

```
shift +1: -1→0, 0→1, 1→-1
shift -1: -1→1, 0→-1, 1→0
shift  0: identity
```

The `rem_euclid` ensures correct behavior for negative shifts. Rotation by 3 is the identity.

### fold(signal, threshold)

Wavefolding. Values within `[-threshold, threshold]` pass through. Values exceeding the threshold are reflected:

```
v > threshold:      output = 2·threshold - v     (fold down)
v < -threshold:     output = -2·threshold - v    (fold up)
threshold = 0:      everything → 0                (silence)
```

### quantize(signal, levels)

Snap to `levels` evenly-spaced steps centered at 0. Levels=1 maps everything to 0. The step size is `255 / (levels * 2)`, and values are rounded to the nearest step.

## Code Example

```rust
use ternary_bite::*;

// Bit crush: temporal resolution reduction
let signal = vec![10, 20, 30, 40, 50, 60, 70, 80];
let crushed = crush(&signal, 4);
assert_eq!(crushed, vec![10, 10, 10, 10, 50, 50, 50, 50]);

// Downsample: block averaging
let data = vec![10, 20, 30, 40];
let down = downsample(&data, 2);
assert_eq!(down, vec![15, 35]);

// Bit rotate: cyclic permutation through Z₃
let ternary = vec![-1i8, 0, 1];
let rotated = bit_rotate(&ternary, 1);
assert_eq!(rotated, vec![0, 1, -1]);

let back = bit_rotate(&rotated, -1);
assert_eq!(back, vec![-1, 0, 1]); // round-trip

// Wavefold: mirror reflection past threshold
let folded = fold(&[80], 50);
assert_eq!(folded, vec![20]); // 2*50 - 80 = 20

let neg_folded = fold(&[-80], 50);
assert_eq!(neg_folded, vec![-20]); // -2*50 - (-80) = -20

// Quantize: reduce amplitude resolution
let silenced = quantize(&[10, 20, 30], 1);
assert_eq!(silenced, vec![0, 0, 0]);
```

### Pipeline: Coarse-grain a signal

```rust
use ternary_bite::*;

let raw: Vec<i8> = (0..100).map(|i| ((i * 7 + 3) % 5 - 2) as i8).collect();

let crushed = crush(&raw, 10);        // reveal temporal structure
let down = downsample(&crushed, 2);   // reduce dimensionality
let ternary: Vec<i8> = down.iter().map(|&v| v.clamp(-1, 1)).collect();
let shifted = bit_rotate(&ternary, 1); // shift perspective

println!("{} → {} → {} samples", raw.len(), crushed.len(), down.len());
// 100 → 100 → 50 samples
```

## Module Map

```
ternary_bite
├── crush(signal: &[i8], rate: usize) → Vec<i8>
│   └── Sample-and-hold. First value per block, held for block duration.
│
├── downsample(signal: &[i8], factor: usize) → Vec<i8>
│   └── Block averaging. Output = ceil(len/factor).
│
├── bit_rotate(signal: &[i8], shift: i32) → Vec<i8>
│   └── Cyclic permutation through Z₃ = {-1, 0, +1}.
│
├── fold(signal: &[i8], threshold: i8) → Vec<i8>
│   └── Wavefolding distortion. Values past threshold are mirrored.
│
└── quantize(signal: &[i8], levels: i8) → Vec<i8>
    └── Snap to `levels` evenly-spaced amplitude steps.
```

## Design Decisions

**Full `i8` range, not just {-1, 0, +1}.** Only `bit_rotate` operates in ternary space. `crush`, `downsample`, `quantize`, and `fold` work on any `i8` signal. This is intentional: these operations are useful for coarse-graining arbitrary signals *toward* ternary. The typical pipeline is: apply destructive transforms to an `i8` signal, then clamp to {-1, 0, +1} at the end.

**`crush` preserves length, `downsample` doesn't.** `crush` outputs the same number of samples — it holds values to reduce within-block variation. `downsample` outputs fewer samples — one per block. They solve different problems: crush for temporal blurring, downsample for dimensionality reduction.

**Integer arithmetic everywhere.** No floating point. `downsample` uses integer division (truncation toward zero). `quantize` uses integer step sizes. `fold` uses integer arithmetic. This avoids rounding surprises and makes the output deterministic across platforms.

**`fold` maps threshold=0 to silence.** When the threshold is zero, every value exceeds it and gets folded to 0. This is mathematically correct (`2·0 - v = -v`, then `-(-v) = v`... actually no — the code returns 0 directly when threshold is 0). The design choice: zero threshold is a degenerate case that should produce a degenerate result (silence).

**No `#![forbid(unsafe_code)]` needed — it's already there.** The crate explicitly forbids unsafe code. All operations are bounds-checked, overflow-checked, and safe.

## Status

| Aspect | State |
|--------|-------|
| crush | Stable, tested |
| quantize | Stable, tested |
| downsample | Stable, tested |
| bit_rotate | Stable, tested |
| fold | Stable, tested |
| Tests | 16 |
| Unsafe | Forbidden (`#![forbid(unsafe_code)]`) |
| MSRV | Edition 2021 |
| Lines of code | ~210 |

**Known limitations:** `quantize` uses a general-purpose step calculation that may not produce clean ternary outputs for all level counts — it's designed for gradual quantization, not direct ternary snapping. `crush` doesn't interpolate between blocks — it's pure sample-and-hold, which introduces stair-step artifacts. `bit_rotate` maps arbitrary `i8` values through the Z₃ group: values outside {-1, 0, +1} will be projected into ternary space, which may or may not be what you want.

## Related Crates

- **[ternary-warp](https://github.com/SuperInstance/ternary-warp)** — Non-destructive transformations (clamp, smooth, differentiate). The yang to this crate's yin.
- **[ternary-gauge](https://github.com/SuperInstance/ternary-gauge)** — Measure signals before and after destructive transforms
- **[ternary-complexity](https://github.com/SuperInstance/ternary-complexity)** — After coarse-graining, measure whether the signal actually simplified
- **[ternary-pool](https://github.com/SuperInstance/ternary-pool)** — 2D spatial pooling (this crate is 1D temporal)

## License

MIT
