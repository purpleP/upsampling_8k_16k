import sys
import numpy as np
from scipy import signal
import matplotlib.pyplot as plt

def plot_response(taps, fs, w, h, title):
    fig, (ax, tx) = plt.subplots(2)
    tx.set_title('Taps')
    tx.plot(taps)
    tx.grid(True)
    ax.plot(0.5*fs*w/np.pi, 20*np.log10(np.abs(h)))
    ax.set_xlim(0, 0.5*fs)
    ax.grid(True)
    ax.set_xlabel('Frequency (Hz)')
    ax.set_ylabel('Gain (dB)')
    ax.set_title(title)

fs = 16000.0
trans = 0.03
cutoff = (0.25 - trans) * fs
bands = [0, cutoff, (0.25 + trans) * fs, 0.5 * fs]
desired = [2, 0]
numtaps = 63
# taps = signal.firls(numtaps, [cutoff, fs * 0.5], desired, fs=fs)
taps = signal.remez(numtaps, bands, desired, [1, 1], fs=fs, maxiter=20000)
taps[np.abs(taps) < 1e-5] = 0.0
w, h = signal.freqz(taps, [1], worN=2000)
reversed = taps[::-1]
np.set_printoptions(precision=1000, linewidth=1)
taps_i16 = (taps * 2**15).astype('int16')
taps_i16_reversed = taps_i16[::-1]
print('taps', taps)
print('integer taps', np.array2string(taps_i16))
print('halfband', np.array2string(reversed[0::2], separator=','))
print('halfband integers', np.array2string(taps_i16_reversed[0::2], separator=','))
plot_response(taps, fs, w, h, "Low-pass Filter")
# plt.show()