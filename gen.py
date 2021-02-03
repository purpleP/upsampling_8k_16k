from scipy.io.wavfile import write
import numpy as np
import sys

_, f, out = sys.argv
f = int(f)
samplerate = 8000
t = np.linspace(0., 1., samplerate)
amplitude = np.iinfo(np.int16).max
data = amplitude * np.sin(2. * np.pi * f * t)
data[500:1000] = 32766
write(out, samplerate, data.astype(np.int16))