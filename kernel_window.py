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
nyq_rate = fs / 2
cutoff = 4000.0
trans_width = 100
ripple_db = 60.0
numtaps = 64
N, beta = signal.kaiserord(ripple_db, trans_width / nyq_rate)
print('order', N)
taps = signal.firwin(N, cutoff/nyq_rate, window=('kaiser', beta))
w, h = signal.freqz(taps, [1], worN=2000)
reversed = taps[::-1]
print('taps', taps)
print('reversed', reversed)
print('polyphase', repr(reversed[1::2]), repr(reversed[0::2]))
plot_response(fs, w, h, "Low-pass Filter")
plt.show()