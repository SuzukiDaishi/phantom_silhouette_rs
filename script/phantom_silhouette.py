from typing import Tuple
import sys
import wave

import numpy as np
import pyworld as pw


def hz_to_spec(hz: np.ndarray, sr: int, spec_samples: int) -> np.ndarray:
    """Convert physical frequency to spectrogram coordinate."""
    return (hz / (sr // 2)) * spec_samples


def hz_to_erb(hz: np.ndarray) -> np.ndarray:
    """Convert Hertz to the ERB scale."""
    return 21.4 * np.log(0.00437 * hz + 1) / np.log(10)


def erb_to_hz(erb: np.ndarray) -> np.ndarray:
    """Convert ERB values back to Hertz."""
    return (np.exp(erb / 21.4 * np.log(10)) - 1) / 0.00437


def low_frequency_suppression(sp: np.ndarray, sr: int) -> np.ndarray:
    """Suppress low frequency components."""
    freqs = np.arange(1, sp.shape[1] + 1) * (sr // 2 / sp.shape[1])
    weights = np.where(
        freqs > 1350,
        1.0,
        np.where(freqs > 550, np.abs((freqs - 550) / (1350 - 550)) ** np.e, 0.0),
    )
    return sp * weights[np.newaxis, :]


def high_frequency_emphasis(sp: np.ndarray, sr: int) -> np.ndarray:
    """Emphasize breathiness in the high frequencies."""
    freqs = np.arange(1, sp.shape[1] + 1) * (sr // 2 / sp.shape[1])
    weights = np.where(
        freqs < 1e3,
        1.0,
        np.where(freqs < 1e4, (freqs - 1e3) / (1e4 - 1e3) + 1.0, 2.0),
    )
    return sp * weights[np.newaxis, :]


def convert_noise(f0: np.ndarray) -> np.ndarray:
    """Replace the excitation signal with white noise."""
    return np.random.random(f0.shape[0])


def covert_pink_noise(f0: np.ndarray) -> np.ndarray:
    """Generate pink noise using Paul Kellett's IIR approximation."""
    n = f0.shape[0]

    if not hasattr(covert_pink_noise, "b0"):
        covert_pink_noise.b0 = 0.0
        covert_pink_noise.b1 = 0.0
        covert_pink_noise.b2 = 0.0
        covert_pink_noise.b3 = 0.0
        covert_pink_noise.b4 = 0.0
        covert_pink_noise.b5 = 0.0
        covert_pink_noise.b6 = 0.0

    b0 = covert_pink_noise.b0
    b1 = covert_pink_noise.b1
    b2 = covert_pink_noise.b2
    b3 = covert_pink_noise.b3
    b4 = covert_pink_noise.b4
    b5 = covert_pink_noise.b5
    b6 = covert_pink_noise.b6

    out = np.zeros(n, dtype=np.float64)
    for i in range(n):
        white = np.random.randn()
        b0 = 0.99886 * b0 + white * 0.0555179
        b1 = 0.99332 * b1 + white * 0.0750759
        b2 = 0.96900 * b2 + white * 0.1538520
        b3 = 0.86650 * b3 + white * 0.3104856
        b4 = 0.55000 * b4 + white * 0.5329522
        b5 = -0.7616 * b5 - white * 0.0168980
        sample = b0 + b1 + b2 + b3 + b4 + b5 + b6 + white * 0.5362
        b6 = white * 0.115926
        out[i] = sample

    covert_pink_noise.b0 = b0
    covert_pink_noise.b1 = b1
    covert_pink_noise.b2 = b2
    covert_pink_noise.b3 = b3
    covert_pink_noise.b4 = b4
    covert_pink_noise.b5 = b5
    covert_pink_noise.b6 = b6

    max_abs = np.max(np.abs(out))
    if max_abs > 0:
        out = out / max_abs

    return out


def phantom_silhouette(f0: np.ndarray, sp: np.ndarray, sr: int) -> Tuple[np.ndarray, np.ndarray]:
    """Apply the Phantom Silhouette conversion using white noise."""
    f0_out = convert_noise(f0)
    sp_out = sp.copy()
    sp_out = low_frequency_suppression(sp_out, sr)
    sp_out = high_frequency_emphasis(sp_out, sr)
    sp_out[sp_out == 0] = 1e-8
    return f0_out, sp_out


def phantom_silhouette_pink(f0: np.ndarray, sp: np.ndarray, sr: int) -> Tuple[np.ndarray, np.ndarray]:
    """Apply the Phantom Silhouette conversion using pink noise."""
    f0_out = covert_pink_noise(f0)
    sp_out = sp.copy()
    sp_out = low_frequency_suppression(sp_out, sr)
    sp_out = high_frequency_emphasis(sp_out, sr)
    sp_out[sp_out == 0] = 1e-8
    return f0_out, sp_out


def _read_wav(path: str) -> Tuple[np.ndarray, int]:
    with wave.open(path, "rb") as wf:
        fs = wf.getframerate()
        n = wf.getnframes()
        samples = wf.readframes(n)
        dtype = {1: np.int8, 2: np.int16, 4: np.int32}[wf.getsampwidth()]
        x = np.frombuffer(samples, dtype=dtype).astype(np.float64)
        if wf.getnchannels() > 1:
            x = x.reshape(-1, wf.getnchannels())[:, 0]
        if wf.getsampwidth() == 2:
            x /= 32768.0
        elif wf.getsampwidth() == 1:
            x = (x - 128) / 128.0
        elif wf.getsampwidth() == 4:
            x /= 2147483648.0
    return x, fs


def _write_wav(path: str, data: np.ndarray, fs: int) -> None:
    data_i16 = np.clip(data * 32768.0, -32768, 32767).astype(np.int16)
    with wave.open(path, "wb") as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(fs)
        wf.writeframes(data_i16.tobytes())


def main() -> None:
    if len(sys.argv) < 3:
        print("Usage: python3 phantom_silhouette.py <input.wav> <output.wav> [pink]", file=sys.stderr)
        return
    inp, out = sys.argv[1], sys.argv[2]
    use_pink = len(sys.argv) > 3 and sys.argv[3] == "pink"

    x, fs = _read_wav(inp)
    _f0, t = pw.dio(x, fs)
    f0 = pw.stonemask(x, _f0, t, fs)
    sp = pw.cheaptrick(x, f0, t, fs)
    ap = pw.d4c(x, f0, t, fs)

    if use_pink:
        f0_new, sp_new = phantom_silhouette_pink(f0, sp, fs)
    else:
        f0_new, sp_new = phantom_silhouette(f0, sp, fs)

    y = pw.synthesize(f0_new, sp_new, ap, fs)
    _write_wav(out, y, fs)


if __name__ == "__main__":
    main()
