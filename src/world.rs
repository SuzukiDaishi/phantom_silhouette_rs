use Rust_WORLD::rsworld::{cheaptrick, d4c, dio, stonemask, synthesis};
use Rust_WORLD::rsworld_sys::{CheapTrickOption, D4COption, DioOption};
use crate::phantomsilhouette::phantom_silhouette_pink;

pub fn extract_f0(x: &[f64], fs: i32) -> Vec<f64> {
    let option = DioOption::new();
    let (time_axis, f0) = dio(&x.to_vec(), fs, &option);
    let refined = stonemask(&x.to_vec(), fs, &time_axis, &f0);
    refined
}

/// Apply the Phantom Silhouette conversion with pink noise using WORLD.
pub fn phantom_silhouette_signal(x: &[f64], fs: i32) -> Vec<f64> {
    let dio_opt = DioOption::new();
    let (time, f0) = dio(&x.to_vec(), fs, &dio_opt);
    let f0 = stonemask(&x.to_vec(), fs, &time, &f0);

    let mut ct_opt = CheapTrickOption::new(fs);
    let sp = cheaptrick(&x.to_vec(), fs, &time, &f0, &mut ct_opt);
    let d4c_opt = D4COption::new();
    let ap = d4c(&x.to_vec(), fs, &time, &f0, &d4c_opt);

    let (f0_new, sp_new) = phantom_silhouette_pink(&f0, &sp, fs as usize);

    let mut y = synthesis(&f0_new, &sp_new, &ap, dio_opt.frame_period, fs);
    if y.len() > x.len() {
        y.truncate(x.len());
    }
    y
}
