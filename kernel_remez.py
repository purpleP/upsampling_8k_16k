import numpy as np
from scipy import signal
import matplotlib.pyplot as plt

def plot_response(fs, w, h, title):
    "Utility function to plot response functions"
    fig = plt.figure()
    ax = fig.add_subplot(111)
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
taps[abs(taps) <= 1e-4] = 0.
w, h = signal.freqz(taps, [1], worN=2000)
reversed = taps[::-1]
np.set_printoptions(precision=20)
print('taps', taps)
print('polyphase', np.array2string(reversed[0::2], separator=','))
plot_response(fs, w, h, "Low-pass Filter")
# plt.show()