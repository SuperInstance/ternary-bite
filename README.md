# ternary-bite

**Bit crushing, wavefolding, and lo-fi degradation for ternary signals.**

In a world of pristine 24-bit audio, there's a peculiar beauty in *destroying* resolution. Bit crushing takes a detailed signal and smashes it into fewer levels. Downsampling stretches time. Wavefolding folds energy back on itself instead of clipping.

This crate does all of that for signals in `{-1, 0, +1}` — the ternary domain. The constraint makes the effects more extreme and the math more interesting. When you only have three values, every degradation technique becomes a structural transformation, not just a filter.

## What's Inside

- **`crush(signal, rate)`** — sample-and-hold at reduced rate. Every Nth sample holds, creating that classic lo-fi stutter
- **`quantize(signal, levels)`** — reduce to fewer amplitude levels
- **`downsample(signal, factor)`** — average blocks, shrink temporal resolution
- **`bit_rotate(signal, shift)`** — rotate values through ternary space (cyclic permutation of {-1, 0, +1})
- **`fold(signal, threshold)`** — wavefolding distortion: values that exceed threshold fold back instead of clipping
- **`wrap(signal)`** — modular wrapping in ternary arithmetic

## Quick Example

```rust
use ternary_bite::*;

let signal = vec![-1, 0, 1, -1, 0, 1, -1, 0, 1];

// Bit crush: hold every 3rd sample
let crushed = crush(&signal, 3);
// [-1, -1, -1, -1, -1, -1, -1, -1, -1] — all held to first value

// Wavefold: fold values beyond threshold back
let folded = fold(&[1, 1, 1, 0, -1], 0);
// Energy reflected back into the signal

// Bit rotate: cycle through ternary space
let rotated = bit_rotate(&signal, 1);
// Each value shifted: -1→0, 0→1, 1→-1
```

## Why Ternary Degradation?

**Three values make every effect decisive.** There's no "slightly distorted" — the signal either changes state or it doesn't. This makes ternary signal processing a laboratory for understanding what degradation *does* at a structural level, stripped of continuous nuance.

**Use cases:**
- **Audio effect design** — lo-fi, bit-crush, and wavefold with minimal state
- **Generative art** — controlled destruction as a creative tool
- **Data augmentation** — degrade signals for robust ML training
- **Glitch aesthetics** — ternary constraints produce distinctive artifacts
- **Signal processing education** — see degradation mechanics without floating-point noise

## Install

```bash
cargo add ternary-bite
```

## License

MIT
