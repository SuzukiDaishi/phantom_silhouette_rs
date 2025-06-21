use crate::noise::{pink_noise, white_noise};
use crate::spectral::{high_frequency_emphasis, low_frequency_suppression};

/// Apply the Phantom Silhouette conversion using white noise.
/// Returns the new excitation and modified spectral envelope.
pub fn phantom_silhouette(f0: &[f64], sp: &[Vec<f64>], sr: usize) -> (Vec<f64>, Vec<Vec<f64>>) {
    let f0_out = white_noise(f0.len());
    let mut sp_out = sp.to_vec();
    sp_out = low_frequency_suppression(sp_out, sr);
    sp_out = high_frequency_emphasis(sp_out, sr);
    (f0_out, sp_out)
}

/// Apply the Phantom Silhouette conversion using pink noise.
pub fn phantom_silhouette_pink(
    f0: &[f64],
    sp: &[Vec<f64>],
    sr: usize,
) -> (Vec<f64>, Vec<Vec<f64>>) {
    let f0_out = pink_noise(f0.len());
    let mut sp_out = sp.to_vec();
    sp_out = low_frequency_suppression(sp_out, sr);
    sp_out = high_frequency_emphasis(sp_out, sr);
    (f0_out, sp_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phantom_silhouette_shapes() {
        let f0 = vec![0.0; 256];
        let sp = vec![vec![1.0; 128]; 256];
        let (f0_out, sp_out) = phantom_silhouette(&f0, &sp, 48000);
        assert_eq!(f0_out.len(), f0.len());
        assert_eq!(sp_out.len(), sp.len());
        assert_eq!(sp_out[0].len(), sp[0].len());
    }
}
