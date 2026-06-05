# ternary-bite

**The beautiful destruction of resolution. Bit crushing, wavefolding, and the art of sounding worse on purpose.**

In a world of 24-bit, 96kHz pristine audio, there's a peculiar beauty in *destroying* resolution. Bit crushing takes a detailed signal and smashes it into fewer levels. Downsampling stretches time until you can hear the individual steps. Wavefolding folds energy back on itself instead of clipping — creating harmonics that weren't there, harmonics that *shouldn't* be there, harmonics that sound like a monster waking up.

This crate is the sound of digital decay. It's why old video games sounded like nothing else. It's why producers pay good money for plugins that make their tracks sound *worse*. Because "worse" is character. "Worse" is texture. "Worse" is the difference between a clean recording and something that *moves* you.

In ternary, bit crushing means collapsing to {-1, 0, +1} — which is already the lowest possible resolution. So what does "destroying" a ternary signal even mean? It means *downsampling in time* (fewer samples per cycle, more staircase), *wavefolding* (folding values back when they exceed bounds), and *rotate/crush* transforms that mangle the signal's shape in ways that sound like broken machines singing.

## What's Inside

- **`crush(signal, levels)`** — reduce to fewer quantization levels. At 3 levels, you get pure ternary. At 2, binary. At 1, silence
- **`downsample(signal, factor)`** — hold each value for N ticks. The sound of sample-rate reduction — that classic "digitization" artifact
- **`wavefold(signal, threshold)`** — instead of hard clipping, fold the signal back. Energy creates new harmonics. Louder = more complex = more interesting
- **`rotate(signal, amount)`** — cyclically rotate the ternary values. -1→0→+1→-1. Z₃ as an audio effect
- **`rectify(signal)`** — flip negative values positive. Harmonics double. The sound gets brighter and more aggressive
- **`saturate(signal, drive)`** — soft saturation that pushes values toward ±1. Warmth through distortion

## Quick Example

```rust
use ternary_bite::*;

let clean = vec![1, 0, -1, 0, 1, 0, -1, 0];

// Downsample: hold each value for 2 ticks
let chunky = downsample(&clean, 2);
// [1, 1, 0, 0, -1, -1, 0, 0] — staircase artifacts

// Wavefold at threshold 0.5
let folded = wavefold(&clean, 0.5);
// Values that exceed 0.5 fold back — new harmonics emerge

// Rotate the signal through Z₃
let rotated = rotate(&clean, 1);
// Every value shifts: 1→-1, 0→1, -1→0. Same rhythm, different pitch.

// Saturate: push toward the extremes
let hot = saturate(&clean, 2.0);
// The quiet parts get louder. The loud parts hit the ceiling.
```

## The Deeper Truth

**Destruction is a creative act.** Every audio effect that producers love — distortion, saturation, bitcrushing, waveshaping — is fundamentally about *removing information*. You start with a clean, detailed signal and you *destroy* parts of it. The parts that survive become more important. The artifacts of destruction become the character.

In ternary, the signal is already at minimum information — three levels. So "bit crushing" can't reduce the bit depth further. Instead, ternary destruction operates in *time* (downsampling) and *shape* (wavefolding, rotation). The constraint forces creativity: when you can't just "add more bits," you have to find new ways to mangle what you have.

Wavefolding is the star. In continuous audio, wavefolding creates harmonic series that get richer the harder you push. In ternary, wavefolding creates *patterns* — the folding operation maps the three states onto themselves in non-obvious ways, producing rhythmic and harmonic structures that are impossible to get from any other process. It's controlled chaos: deterministic destruction with emergent beauty.

**Use cases:**
- **Lo-fi production** — the authentic sound of digital degradation
- **Sound design** — generate textures that are impossible with continuous processing
- **Game audio** — retro, chiptune, and "broken machine" aesthetics
- **Live performance** — mangle signals in real-time with Z₃ rotation
- **Education** — hear what sample rate and bit depth actually *mean*

## See Also

- **ternary-wave** — the clean signals you're destroying
- **ternary-echo** — echo + bite = dub techno heaven
- **ternary-gate** — gating after crushing creates rhythmic stutter effects
- **ternary-sampler** — sample and destroy
- **ternary-rack** — wire bite effects into a modular signal chain
- **ternary-needledrop** — a different kind of degradation (vinyl warmth, not digital cold)

## Install

```bash
cargo add ternary-bite
```

## License

MIT
