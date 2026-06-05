#![forbid(unsafe_code)]

/// Bit crushing: reduce resolution by sampling every Nth value
pub fn crush(signal: &[i8], rate: usize) -> Vec<i8> {
    if rate == 0 {
        return signal.to_vec();
    }
    signal
        .chunks(rate)
        .flat_map(|chunk| {
            let hold = *chunk.first().unwrap_or(&0);
            vec![hold; chunk.len()]
        })
        .collect()
}

/// Quantize: snap ternary values to fewer levels
pub fn quantize(signal: &[i8], levels: i8) -> Vec<i8> {
    if levels <= 1 {
        return vec![0; signal.len()];
    }
    signal
        .iter()
        .map(|&s| {
            // Map to levels centered at 0
            let step = 255i32 / (levels as i32 * 2);
            if step == 0 {
                return 0;
            }
            let quantized = ((s as i32 + 128) / step as i32) * step as i32 - 128;
            quantized.clamp(-128, 127) as i8
        })
        .collect()
}

/// Downsample: reduce temporal resolution by averaging blocks
pub fn downsample(signal: &[i8], factor: usize) -> Vec<i8> {
    if factor <= 1 {
        return signal.to_vec();
    }
    signal
        .chunks(factor)
        .map(|chunk| {
            let sum: i32 = chunk.iter().map(|&v| v as i32).sum();
            (sum / chunk.len() as i32).clamp(-128, 127) as i8
        })
        .collect()
}

/// Bit rotate: rotate ternary values by shift positions
pub fn bit_rotate(signal: &[i8], shift: i32) -> Vec<i8> {
    signal
        .iter()
        .map(|&s| {
            let s = s as i32;
            // Rotate in ternary space {-1, 0, 1}
            let rotated = if shift % 3 == 0 {
                s
            } else {
                // circular shift through ternary values
                let normalized = s + 1; // 0, 1, 2
                let rotated_val = ((normalized + shift).rem_euclid(3)) - 1;
                rotated_val
            };
            rotated.clamp(-128, 127) as i8
        })
        .collect()
}

/// Wavefolding distortion: fold values that exceed threshold back
pub fn fold(signal: &[i8], threshold: i8) -> Vec<i8> {
    signal
        .iter()
        .map(|&s| {
            let t = threshold as i32;
            if t == 0 {
                return 0;
            }
            let folded = if s as i32 > t {
                2 * t - s as i32
            } else if (s as i32) < -(t) {
                -(2 * t) - s as i32
            } else {
                s as i32
            };
            folded.clamp(-128, 127) as i8
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crush_rate_1() {
        let sig = &[1, 2, 3, 4];
        let out = crush(sig, 1);
        assert_eq!(out, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_crush_rate_2() {
        let sig = &[10, 20, 30, 40];
        let out = crush(sig, 2);
        assert_eq!(out, &[10, 10, 30, 30]);
    }

    #[test]
    fn test_crush_rate_4() {
        let sig = &[5, 6, 7, 8, 9, 10, 11, 12];
        let out = crush(sig, 4);
        assert_eq!(out, &[5, 5, 5, 5, 9, 9, 9, 9]);
    }

    #[test]
    fn test_crush_rate_0_passthrough() {
        let sig = &[1, 2, 3];
        let out = crush(sig, 0);
        assert_eq!(out, sig);
    }

    #[test]
    fn test_quantize_levels_1() {
        let sig = &[10, 20, 30];
        let out = quantize(sig, 1);
        assert_eq!(out, &[0, 0, 0]);
    }

    #[test]
    fn test_quantize_levels_3() {
        let sig = &[0, 64, -64];
        let out = quantize(sig, 3);
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn test_downsample_factor_1() {
        let sig = &[1, 2, 3];
        let out = downsample(sig, 1);
        assert_eq!(out, sig);
    }

    #[test]
    fn test_downsample_factor_2() {
        let sig = &[10, 20, 30, 40];
        let out = downsample(sig, 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], 15); // (10+20)/2
        assert_eq!(out[1], 35); // (30+40)/2
    }

    #[test]
    fn test_downsample_uneven() {
        let sig = &[10, 20, 30];
        let out = downsample(sig, 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], 15);
        assert_eq!(out[1], 30);
    }

    #[test]
    fn test_bit_rotate_zero() {
        let sig = &[-1, 0, 1];
        let out = bit_rotate(sig, 0);
        assert_eq!(out, &[-1, 0, 1]);
    }

    #[test]
    fn test_bit_rotate_one() {
        let sig = &[-1i8, 0, 1];
        let out = bit_rotate(sig, 1);
        assert_eq!(out, &[0, 1, -1]);
    }

    #[test]
    fn test_bit_rotate_neg_one() {
        let sig = &[-1i8, 0, 1];
        let out = bit_rotate(sig, -1);
        assert_eq!(out, &[1, -1, 0]);
    }

    #[test]
    fn test_fold_below_threshold() {
        let sig = &[5, -5, 0];
        let out = fold(sig, 50);
        assert_eq!(out, &[5, -5, 0]);
    }

    #[test]
    fn test_fold_above_threshold() {
        let sig = &[80];
        let out = fold(sig, 50);
        assert_eq!(out, &[20]); // 2*50 - 80 = 20
    }

    #[test]
    fn test_fold_below_neg_threshold() {
        let sig = &[-80];
        let out = fold(sig, 50);
        assert_eq!(out, &[-20]); // -2*50 - (-80) = -20
    }

    #[test]
    fn test_fold_zero_threshold() {
        let sig = &[10, -10, 0];
        let out = fold(sig, 0);
        assert_eq!(out, &[0, 0, 0]);
    }
}
