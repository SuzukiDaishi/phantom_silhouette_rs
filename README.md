# Phantom Silhouette Rs

※ これは失敗作です

## Building

After installing [Rust](https://rustup.rs/), you can compile Phantom Silhouette Rs as follows:

```shell
cargo xtask bundle phantom_silhouette_rs --release
```

## WORLD comparison test

A small test verifies that the WORLD vocoder bindings in Rust produce the same fundamental frequency values as `pyworld`. After installing the `numpy` and `pyworld` Python packages, run:

```shell
cargo test --test world_integration -- --nocapture
```

This invokes `script/extract_f0.py` on `test_sample/a01.wav` and compares its output with the Rust implementation.
Only voiced frames (where the extracted F0 is non‑zero) are taken into account for the comparison.

## Phantom Silhouette script

A small Python script in `script/phantom_silhouette.py` applies the Phantom Silhouette voice conversion. It replaces the excitation signal with white or pink noise and adjusts the spectral envelope without formant shifting.

Run it as follows to process a wav file:

```shell
python3 script/phantom_silhouette.py input.wav output.wav [pink]
```

Add `pink` to use pink noise instead of white noise.

## Plugin parameters

The VST plugin exposes one control:

- **Mix** – Cross‑fades between the incoming audio and the processed
  Phantom Silhouette signal. `1.0` leaves the original untouched, while
  `0.0` outputs only the converted whisper.

The plugin now performs the full WORLD‑based Phantom Silhouette conversion
internally. Each buffer is analyzed with WORLD, the excitation is replaced with
pink noise, and the modified spectral envelope is resynthesized before being
blended with the input signal.

## VST runtime test

`script/test_vst.py` loads the compiled plugin with pedalboard and runs it on
`test_sample/a01.wav` for a few sample rates and channel counts.  It prints the
elapsed processing time for each run.  Run it after bundling the plugin:

```shell
python3 script/test_vst.py
```
