/// Spectral processing helpers used by the Phantom Silhouette algorithm.

/// Suppress low frequency components.
pub fn low_frequency_suppression(mut sp: Vec<Vec<f64>>, sr: usize) -> Vec<Vec<f64>> {
    let len = sp.get(0).map_or(0, |row| row.len());
    let step = sr as f64 / 2.0 / len as f64;
    for row in &mut sp {
        for (i, val) in row.iter_mut().enumerate() {
            let f = (i + 1) as f64 * step;
            let w = if f > 1350.0 {
                1.0
            } else if f > 550.0 {
                ((f - 550.0) / (1350.0 - 550.0))
                    .abs()
                    .powf(std::f64::consts::E)
            } else {
                0.0
            };
            *val *= w;
        }
    }
    sp
}

/// Emphasize high frequency breathiness.
pub fn high_frequency_emphasis(mut sp: Vec<Vec<f64>>, sr: usize) -> Vec<Vec<f64>> {
    let len = sp.get(0).map_or(0, |row| row.len());
    let step = sr as f64 / 2.0 / len as f64;
    for row in &mut sp {
        for (i, val) in row.iter_mut().enumerate() {
            let f = (i + 1) as f64 * step;
            let w = if f < 1000.0 {
                1.0
            } else if f < 10000.0 {
                (f - 1000.0) / (10000.0 - 1000.0) + 1.0
            } else {
                2.0
            };
            *val *= w;
        }
    }
    sp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_shape() {
        let sp = vec![vec![1.0; 16]; 4];
        let sup = low_frequency_suppression(sp.clone(), 48000);
        assert_eq!(sup.len(), sp.len());
        assert_eq!(sup[0].len(), sp[0].len());
        let emp = high_frequency_emphasis(sp.clone(), 48000);
        assert_eq!(emp[0].len(), sp[0].len());
    }
}
