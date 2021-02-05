import sys
import numpy as np
from scipy import signal
import matplotlib.pyplot as plt


def plot_response(fs, taps, w, h, input, filtered, title):
    fig, (ax, tx, sx, fx) = plt.subplots(4)
    ax.grid(True)
    tx.grid(True)
    sx.grid(True)
    fx.grid(True)
    sx.set_title('Signal')
    fx.set_title('Filtered')
    sx.plot(input)
    fx.plot(filtered)
    tx.set_title('Taps')
    tx.plot(taps)
    ax.plot(0.5 * fs * w / np.pi, 20 * np.log10(np.abs(h)))
    ax.set_xlim(0, 0.5 * fs)
    ax.set_xlabel('Frequency (Hz)')
    ax.set_ylabel('Gain (dB)')
    ax.set_title(title)


def filter():
    samplerate = 8000
    len_sec = 1
    t = np.linspace(0., len_sec, 200)
    f = 1000
    amplitude = np.iinfo(np.int16).max
    data = amplitude * np.sin(2 * np.pi * f * t)
    data[:100:2] = 2**15 - 1
    data[1:100:2] = -32768
    z = np.zeros(len(data))
    with_zeros = np.dstack((data, z)).reshape((400))
    taps, w, h = gen_taps()
    filtered = signal.lfilter(taps, 1.0, with_zeros)
    # fig, (ax) = plt.subplots()
    # ax.grid(True)
    # plt.hist(filtered, bins=[*range(0, 100000, 10000)])
    np.savetxt('filtered.csv', filtered, delimiter='\t')
    plot_response(
        samplerate, taps, w, h, data, filtered, 'Half-band Interpolation Filter'
    )
    plt.show()


def gen_taps():
    fs = 16000.0
    trans = 0.03
    cutoff = (0.25 - trans) * fs
    bands = [0, cutoff, (0.25 + trans) * fs, 0.5 * fs]
    desired = [2, 0]
    numtaps = 63
    # taps = signal.firls(numtaps, [cutoff, fs * 0.5], desired, fs=fs)
    taps = signal.remez(numtaps, bands, desired, [1, 1], fs=fs, maxiter=20000)
    w, h = signal.freqz(taps, [1], worN=2000)
    reversed = taps[::-1]
    np.set_printoptions(precision=1000, linewidth=1)
    taps_i16 = (taps * 2**14).astype('int16')
    taps_i16_reversed = taps_i16[::-1]
    ts = taps_i16_reversed[::2]
    sum_of_positive_coefficients = ts[ts >= 0].sum()
    sum_of_negative_coefficients = ts[ts < 0].sum()
    min_output = (
        sum_of_positive_coefficients * -(2**15 - 1) + sum_of_negative_coefficients *
        (2**15 - 1)
    )
    max_output = (
        sum_of_positive_coefficients * (2**15 - 1) +
        sum_of_negative_coefficients * -(2**15 - 1)
    )
    return taps, w, h
    # print('taps', taps)
    # print('integer taps', np.array2string(taps_i16))
    # print('halfband', np.array2string(reversed[0::2], separator=','))
    # print(
    #     'halfband integers',
    #     np.array2string(taps_i16_reversed[0::2], separator=',')
    # )
    # print('sum of positive', sum_of_positive_coefficients)
    # print('sum of negative', sum_of_negative_coefficients)
    # print('min output', min_output)
    # print('max output', max_output)
    # plot_response(taps, fs, w, h, 'Low-pass Filter')
    # plt.show()


if __name__ == '__main__':
    filter()
