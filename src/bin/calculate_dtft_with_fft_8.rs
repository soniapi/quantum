use num_complex::Complex;
use quantum::{fft_item_n, dtft};

fn main() {
    let mut seq: Vec<Complex<f64>> = Vec::new();

    seq.push(Complex::new(1.0, 2.0));
    seq.push(Complex::new(-0.5, 1.5));
    seq.push(Complex::new(2.0, -1.0));
    seq.push(Complex::new(1.0, 0.5));
    seq.push(Complex::new(-1.0, -2.0));
    seq.push(Complex::new(0.5, -1.5));
    seq.push(Complex::new(-2.0, 1.0));
    seq.push(Complex::new(-1.0, -0.5));

    println!("FFT Inner loop for n from 0 to 1");
    let output_dtft_item_0 = fft_item_n(&seq, 0, 0) + fft_item_n(&seq, 0, 1) + fft_item_n(&seq, 0, 2) + fft_item_n(&seq, 0, 3);
    let output_dtft_item_1 = fft_item_n(&seq, 1, 0) + fft_item_n(&seq, 1, 1) + fft_item_n(&seq, 1, 2) + fft_item_n(&seq, 1, 3);
    let output_dtft_item_2 = fft_item_n(&seq, 2, 0) + fft_item_n(&seq, 2, 1) + fft_item_n(&seq, 2, 2) + fft_item_n(&seq, 2, 3);
    let output_dtft_item_3 = fft_item_n(&seq, 3, 0) + fft_item_n(&seq, 3, 1)+ fft_item_n(&seq, 3, 2) + fft_item_n(&seq, 3, 3);
    let output_dtft_item_4 = fft_item_n(&seq, 4, 0) + fft_item_n(&seq, 4, 1) + fft_item_n(&seq, 4, 2) + fft_item_n(&seq, 4, 3);
    let output_dtft_item_5 = fft_item_n(&seq, 5, 0) + fft_item_n(&seq, 5, 1) + fft_item_n(&seq, 5, 2) + fft_item_n(&seq, 5, 3);
    let output_dtft_item_6 = fft_item_n(&seq, 6, 0) + fft_item_n(&seq, 6, 1) + fft_item_n(&seq, 6, 2) + fft_item_n(&seq, 6, 3);
    let output_dtft_item_7 = fft_item_n(&seq, 7, 0) + fft_item_n(&seq, 7, 1)+ fft_item_n(&seq, 7, 2) + fft_item_n(&seq, 7, 3);
    
    let output_dtft = dtft(&seq);

    println!(
        "FFT: Output of dtft items in series: {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}",
        output_dtft_item_0, output_dtft_item_1, output_dtft_item_2, output_dtft_item_3, output_dtft_item_4, output_dtft_item_5, output_dtft_item_6, output_dtft_item_7,
    );
    println!("FFT: Output dtft series: {:#.2?}", output_dtft);
}
