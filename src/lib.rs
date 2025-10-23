use num_complex::Complex;
use std::f64::consts::PI;

pub fn dtft_item_k(input_seq: &[Complex<f64>], k: i32) -> Complex<f64> {
    let mut n = 0;
    let mut output = 0.0 + Complex::<f64>::i() * 0.0;
    input_seq.iter().for_each(|c: &Complex<f64>| {
        let number: Complex<f64> = Complex::<f64>::i()
            .scale(2.0)
            .scale(-1.0)
            .scale(PI)
            .scale(k as f64 / input_seq.len() as f64)
            .scale(n as f64);
        let exponential: Complex<f64> = number.exp();
        let complex_mult: Complex<f64> = *c * exponential as Complex<f64>;
        output = output + complex_mult;
        n += 1;
    });
    output
}

pub fn dtft(input_seq: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut output_seq = Vec::new();

    for k in 0..input_seq.len() {
        let output = dtft_item_k(&input_seq, k as i32);
        output_seq.push(output);
    }
    output_seq
}

pub fn inv_dtft_item_n(input_seq: &[Complex<f64>], n: i32) -> Complex<f64> {
    let mut k = 0;
    let mut output = 0.0 + Complex::<f64>::i() * 0.0;
    input_seq.iter().for_each(|c: &Complex<f64>| {
        let number: Complex<f64> = Complex::<f64>::i()
            .scale(2.0)
            .scale(PI)
            .scale(k as f64 / input_seq.len() as f64)
            .scale(n as f64);
        let exponential: Complex<f64> = number.exp();
        let complex_mult: Complex<f64> = *c * exponential as Complex<f64>;
        output = output + complex_mult;
        k += 1;
    });
    output / input_seq.len() as f64
}

pub fn inv_dtft(input_seq: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let mut output_seq = Vec::new();

    for n in 0..input_seq.len() {
        let output = inv_dtft_item_n(&input_seq, n as i32);
        output_seq.push(output);
    }
    output_seq
}
