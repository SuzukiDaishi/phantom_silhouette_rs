use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, StandardNormal};

/// Streaming pink noise generator based on Paul Kellett's IIR approximation.
#[derive(Debug, Clone)]
pub struct PinkNoiseGen {
    b: [f64; 7],
}

impl PinkNoiseGen {
    pub fn new() -> Self {
        Self { b: [0.0; 7] }
    }

    /// Generate a single pink noise sample in the range [-1.0, 1.0].
    pub fn next(&mut self, rng: &mut impl Rng) -> f64 {
        let white: f64 = rng.sample(StandardNormal);
        self.b[0] = 0.99886 * self.b[0] + white * 0.0555179;
        self.b[1] = 0.99332 * self.b[1] + white * 0.0750759;
        self.b[2] = 0.96900 * self.b[2] + white * 0.1538520;
        self.b[3] = 0.86650 * self.b[3] + white * 0.3104856;
        self.b[4] = 0.55000 * self.b[4] + white * 0.5329522;
        self.b[5] = -0.7616 * self.b[5] - white * 0.0168980;
        let sample = self.b[0]
            + self.b[1]
            + self.b[2]
            + self.b[3]
            + self.b[4]
            + self.b[5]
            + self.b[6]
            + white * 0.5362;
        self.b[6] = white * 0.115926;

        sample
    }
}

/// Generate uniform white noise of length `n`.
pub fn white_noise(n: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.sample(Standard)).collect()
}

/// Generate pink noise using Paul Kellett's IIR approximation.
/// The internal filter state is preserved across calls.
pub fn pink_noise(n: usize) -> Vec<f64> {
    static mut STATE: [f64; 7] = [0.0; 7];
    let mut rng = thread_rng();
    let mut out = vec![0.0; n];

    for i in 0..n {
        let white: f64 = StandardNormal.sample(&mut rng);
        unsafe {
            STATE[0] = 0.99886 * STATE[0] + white * 0.0555179;
            STATE[1] = 0.99332 * STATE[1] + white * 0.0750759;
            STATE[2] = 0.96900 * STATE[2] + white * 0.1538520;
            STATE[3] = 0.86650 * STATE[3] + white * 0.3104856;
            STATE[4] = 0.55000 * STATE[4] + white * 0.5329522;
            STATE[5] = -0.7616 * STATE[5] - white * 0.0168980;
            let sample = STATE[0]
                + STATE[1]
                + STATE[2]
                + STATE[3]
                + STATE[4]
                + STATE[5]
                + STATE[6]
                + white * 0.5362;
            STATE[6] = white * 0.115926;
            out[i] = sample;
        }
    }

    let max = out.iter().fold(0.0_f64, |a, &b| a.max(b.abs()));
    if max > 0.0 {
        for v in &mut out {
            *v /= max;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_noise_range() {
        let n = 1000;
        let noise = white_noise(n);
        assert_eq!(noise.len(), n);
        for &v in &noise {
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn test_pink_noise_range() {
        let n = 1024;
        let noise = pink_noise(n);
        let max = noise.iter().cloned().fold(f64::MIN, f64::max);
        let min = noise.iter().cloned().fold(f64::MAX, f64::min);
        assert!(max <= 1.0 && min >= -1.0);
    }
}
