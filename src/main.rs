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

fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn polyphase_fir_filter_8000_to_16000(a: &[f64], kernel: &[f64], out: &mut [f64]) {
    a.windows(32)
        .flat_map(|w| {
            let i = unsafe { w.get_unchecked(15) };
            iter::once(*i).chain(iter::once(dot_product(w, kernel)))
        })
        .enumerate()
        .for_each(|(i, x)| out[i] = x)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let split_kernel = [
        -0.0009302717143512643,
        0.0014245757701187292,
        -0.002460787836795292,
        0.003943971607032482,
        -0.005989992008119632,
        0.00874187972461201,
        -0.012372412653986187,
        0.017111662990475538,
        -0.023280007063154706,
        0.031371582397608344,
        -0.04221487621722491,
        0.05737093606609404,
        -0.080191329559525,
        0.11944349110699277,
        -0.20739640614492658,
        0.6350028611580476,
        0.6350028611580476,
        -0.20739640614492658,
        0.11944349110699277,
        -0.080191329559525,
        0.05737093606609404,
        -0.04221487621722491,
        0.031371582397608344,
        -0.023280007063154706,
        0.017111662990475538,
        -0.012372412653986187,
        0.00874187972461201,
        -0.005989992008119632,
        0.003943971607032482,
        -0.002460787836795292,
        0.0014245757701187292,
        -0.0009302717143512643,
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
    let mut padding = vec![0.0; 31];
    let samples: Result<Vec<f64>, _> = reader
        .samples::<i16>()
        .map(|sample| sample.map(|s| s as f64))
        .collect();
    let samples = samples.unwrap();
    padding.extend_from_slice(&samples[..]);
    let mut out: Vec<f64> = vec![0.0; samples.len() * 2];
    polyphase_fir_filter_8000_to_16000(&padding[..], &split_kernel[..], &mut out[..]);
    for sample in out {
        writer.write_sample::<i16>(sample as i16).unwrap();
    }
}
