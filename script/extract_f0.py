import wave
import sys
import numpy as np
import pyworld as pw

path = sys.argv[1]
with wave.open(path, 'rb') as wf:
    fs = wf.getframerate()
    n = wf.getnframes()
    samples = wf.readframes(n)
    dtype = {1: np.int8, 2: np.int16, 4: np.int32}[wf.getsampwidth()]
    x = np.frombuffer(samples, dtype=dtype).astype(np.float64)
    if wf.getnchannels() > 1:
        x = x.reshape(-1, wf.getnchannels())[:,0]
    if wf.getsampwidth() == 2:
        x /= 32768.0
    elif wf.getsampwidth() == 1:
        x = (x - 128) / 128.0
    elif wf.getsampwidth() == 4:
        x /= 2147483648.0
_f0, t = pw.dio(x, fs)
f0 = pw.stonemask(x, _f0, t, fs)
print(" ".join(f"{v:.6f}" for v in f0))
