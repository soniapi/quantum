use num_complex::Complex;
use quantum::{dtft, dtft_item_k, inv_dtft, inv_dtft_item_n};

fn main() {
    let mut seq: Vec<Complex<f64>> = Vec::new();

    seq.push(Complex::new(1.0, 0.0));
    seq.push(Complex::new(2.0, -1.0));
    seq.push(Complex::new(0.0, -1.0));
    seq.push(Complex::new(-1.0, 2.0));

    let output_dtft_item_0 = dtft_item_k(&seq, 0);
    let output_dtft_item_1 = dtft_item_k(&seq, 1);
    let output_dtft_item_2 = dtft_item_k(&seq, 2);
    let output_dtft_item_3 = dtft_item_k(&seq, 3);

    let output_dtft = dtft(&seq);

    println!(
        "Output of dtft items in series: {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}",
        output_dtft_item_0, output_dtft_item_1, output_dtft_item_2, output_dtft_item_3
    );
    println!("Output dtft series: {:#.2?}", output_dtft);

    let mut inv_seq: Vec<Complex<f64>> = Vec::new();

    inv_seq.push(Complex::new(2.0, 0.0));
    inv_seq.push(Complex::new(-2.0, -2.0));
    inv_seq.push(Complex::new(0.0, -2.0));
    inv_seq.push(Complex::new(4.0, 4.0));

    let output_inv_dtft_item_0 = inv_dtft_item_n(&inv_seq, 0);
    let output_inv_dtft_item_1 = inv_dtft_item_n(&inv_seq, 1);
    let output_inv_dtft_item_2 = inv_dtft_item_n(&inv_seq, 2);
    let output_inv_dtft_item_3 = inv_dtft_item_n(&inv_seq, 3);

    let output_inv_dtft = inv_dtft(inv_seq);

    println!(
        "Output of inv dtft items in series: {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}",
        output_inv_dtft_item_0,
        output_inv_dtft_item_1,
        output_inv_dtft_item_2,
        output_inv_dtft_item_3
    );
    println!("Output inv dtft series: {:#.2?}", output_inv_dtft);
}
