import os
import numpy as np
import soundfile as sf
from pedalboard import Pedalboard, load_plugin
import time

VST_PATH = os.path.join(os.path.dirname(__file__), '..', 'target', 'bundled', 'Phantom Silhouette Rs.vst3')

if not os.path.exists(VST_PATH):
    raise FileNotFoundError(f"VST3 not found at {VST_PATH}. Run 'cargo xtask bundle phantom_silhouette_rs --release' first.")

def resample(audio: np.ndarray, src_sr: int, dst_sr: int) -> np.ndarray:
    if src_sr == dst_sr:
        return audio
    ratio = dst_sr / src_sr
    num_frames = int(round(audio.shape[1] * ratio))
    old_pos = np.linspace(0, 1, audio.shape[1], endpoint=False)
    new_pos = np.linspace(0, 1, num_frames, endpoint=False)
    return np.stack([np.interp(new_pos, old_pos, ch) for ch in audio])

def process(audio: np.ndarray, sr: int, channels: int) -> tuple[np.ndarray, float]:
    """Process audio with the VST while internally converting to mono.

    Returns the processed audio and the time taken in seconds.
    """
    if audio.ndim == 1:
        audio = audio[np.newaxis, :]
    if audio.shape[0] < channels:
        audio = np.vstack([audio] * channels)[:channels]
    else:
        audio = audio[:channels]

    audio = resample(audio, sr, sr)  # ensure correct dtype/resolution (no-op)

    mono = audio.mean(axis=0)
    stereo_input = np.tile(mono, (2, 1))

    board = Pedalboard([load_plugin(VST_PATH)])
    start = time.time()
    stereo_output = board(stereo_input, sr)
    elapsed = time.time() - start
    mono_out = stereo_output.mean(axis=0)

    return np.tile(mono_out, (channels, 1)), elapsed

def main():
    sample_file = os.path.join(os.path.dirname(__file__), '..', 'test_sample', 'a01.wav')
    audio, fs = sf.read(sample_file)
    if audio.ndim == 1:
        audio = audio[np.newaxis, :]
    else:
        audio = audio.T

    for sr in (44100, 48000):
        for ch in (1, 2, 6):
            data = resample(audio, fs, sr)
            out, elapsed = process(data, sr, ch)
            print(f"processed sr={sr} ch={ch} -> {elapsed:.3f}s shape {out.shape}")

if __name__ == '__main__':
    main()
