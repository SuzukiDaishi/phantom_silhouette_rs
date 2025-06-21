use phantom_silhouette_rs::world::extract_f0;
use std::process::Command;

fn read_wav(path: &str) -> (Vec<f64>, i32) {
    let mut reader = hound::WavReader::open(path).unwrap();
    let spec = reader.spec();
    let fs = spec.sample_rate as i32;
    let samples: Vec<f64> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f64 / 32768.0)
        .collect();
    (samples, fs)
}

#[test]
fn compare_f0_with_python() {
    let path = "test_sample/a01.wav";
    let (samples, fs) = read_wav(path);
    let rust_f0 = extract_f0(&samples, fs);

    // Ensure the Python dependencies are available. If not, skip the test.
    let check = Command::new("python3")
        .arg("-c")
        .arg("import numpy, pyworld")
        .output()
        .expect("run python");
    if !check.status.success() {
        eprintln!("skipping python comparison – missing dependencies");
        return;
    }

    let output = Command::new("python3")
        .arg("script/extract_f0.py")
        .arg(path)
        .output()
        .expect("run python");
    assert!(
        output.status.success(),
        "python failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let py_f0: Vec<f64> = stdout
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    assert_eq!(rust_f0.len(), py_f0.len());

    let mut voiced_diffs = Vec::new();
    for (&a, &b) in rust_f0.iter().zip(py_f0.iter()) {
        if a != 0.0 || b != 0.0 {
            voiced_diffs.push((a - b).abs());
        }
    }

    let diff: f64 = voiced_diffs.iter().copied().sum::<f64>() / voiced_diffs.len() as f64;
    assert!(diff < 1e-6, "diff {} too high", diff);
}
