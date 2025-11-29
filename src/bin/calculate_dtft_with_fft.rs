use num_complex::Complex;
use quantum::{fft_item_n, dtft};

fn main() {
    let mut seq: Vec<Complex<f64>> = Vec::new();

    seq.push(Complex::new(1.0, 0.0));
    seq.push(Complex::new(2.0, -1.0));
    seq.push(Complex::new(0.0, -1.0));
    seq.push(Complex::new(-1.0, 2.0));

    println!("FFT Inner loop for n from 0 to 1");
    let output_dtft_item_0 = fft_item_n(&seq, 0, 0) + fft_item_n(&seq, 0, 1);
    let output_dtft_item_1 = fft_item_n(&seq, 1, 0) + fft_item_n(&seq, 1, 1);
    let output_dtft_item_2 = fft_item_n(&seq, 2, 0) + fft_item_n(&seq, 2, 1);
    let output_dtft_item_3 = fft_item_n(&seq, 2, 0) + fft_item_n(&seq, 2, 1);
    
    let output_dtft = dtft(&seq);

    println!(
        "FFT: Output of dtft items in series: {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}",
        output_dtft_item_0, output_dtft_item_1, output_dtft_item_2, output_dtft_item_3
    );
    println!("FFT: Output dtft series: {:#.2?}", output_dtft);
}
