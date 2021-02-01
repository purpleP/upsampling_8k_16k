use hound;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

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
    // let a = a.as_ptr() as *const [f64; 8];
    // let b = b.as_ptr() as *const [f64; 8];
    // let a = unsafe { std::mem::transmute::<_, [f64; 8]>(a) };
    // let b = unsafe { std::mem::transmute::<_, [f64; 8]>(b) };
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn polyphase_fir_filter_8000_to_16000(a: &[f64], split_kernel: &[[f64; 8]], out: &mut [f64]) {
    // let split_kernel = split_kernel.as_ptr() as *const [f64; 16];
    // let split_kernel = unsafe { std::mem::transmute::<_, [[f64; 8]; 2]>(split_kernel) };
    a.windows(8)
        .flat_map(|w|
            split_kernel.iter().map(move |h_i| dot_product(w, h_i))
        )
        .enumerate()
        .for_each(|(i, x)| out[i] = x)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let split_kernel = [
        [
            -0.4509748,
            -0.11602318,
            -0.18187559,
            -0.42433871,
            1.27319174,
            0.25463298,
            0.14139273,
            0.09965497,
        ],
        [
            0.09965497,
            0.14139273,
            0.25463298,
            1.27319174,
            -0.42433871,
            -0.18187559,
            -0.11602318,
            -0.4509748,
        ],
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
    let samples: Result<Vec<f64>, _> = reader
        .samples::<i16>()
        .map(|sample| sample.map(|s| s as f64))
        .collect();
    let samples = samples.unwrap();
    let mut out: Vec<f64> = vec![0.0; samples.len() * 2];
    polyphase_fir_filter_8000_to_16000(&samples[..], &split_kernel[..], &mut out[..]);
    for sample in out {
        writer.write_sample::<i16>(sample as i16).unwrap();
    }
}
