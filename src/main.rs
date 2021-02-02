use hound;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::iter;

// #[target_feature(enable  = "avx2")]
// unsafe fn dot_product(a: &[i16], b: &[i16]) {
//     let a = a.as_ptr() as *const [i16; 16];
//     let b = b.as_ptr() as *const [i16; 16];
//     let a = std::mem::transmute(*a);
//     let b = std::mem::transmute(*b);
//     let ms_256 = _mm256_mullo_epi16(a, b);
//     dbg!(std::mem::transmute::<_, [i16; 16]>(ms_256));
//     let hi_128 = _mm256_castsi256_si128(ms_256);
//     let lo_128 = _mm256_extracti128_si256(ms_256, 1);
//     dbg!(std::mem::transmute::<_, [i16; 8]>(hi_128));
//     dbg!(std::mem::transmute::<_, [i16; 8]>(lo_128));
//     let temp = _mm_add_epi16(hi_128, lo_128);
// }

fn dot_product(a: &[i16], b: &[i16]) -> i32 {
    a.iter()
        .zip(b)
        .map(|(&x, &y)| (x as i32) * (y as i32))
        .sum()
}

fn half_band_fir_filter(a: &[i16], kernel: &[i16], out: &mut [i16]) {
    a.windows(32)
        .flat_map(|w| {
            let i = unsafe { w.get_unchecked(15) };
            iter::once(*i).chain(iter::once((dot_product(w, kernel) >> 15) as i16))
        })
        .enumerate()
        .for_each(|(i, x)| out[i] = x)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let kernel = [
        -30, 46, -80, 129, -196, 286, -405, 560, -762, 1027, -1383, 1879, -2627, 3913, -6795,
        20807, 20807, -6795, 3913, -2627, 1879, -1383, 1027, -762, 560, -405, 286, -196, 129, -80,
        46, -30,
    ];
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("out.wav", spec).unwrap();
    eprintln!("input {}", &input);
    let mut reader = hound::WavReader::open(input).unwrap();
    let mut padding = vec![0; 31];
    let samples: Result<Vec<i16>, _> = reader.samples::<i16>().collect();
    let samples = samples.unwrap();
    padding.extend_from_slice(&samples[..]);
    let mut out: Vec<i16> = vec![0; samples.len() * 2];
    half_band_fir_filter(&padding[..], &kernel[..], &mut out[..]);
    for sample in out {
        writer.write_sample::<i16>(sample as i16).unwrap();
    }
}
