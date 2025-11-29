use num_complex::Complex;
use quantum::{dtft, dtft_item_k};

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

    let output_dtft_item_0 = dtft_item_k(&seq, 0);
    let output_dtft_item_1 = dtft_item_k(&seq, 1);
    let output_dtft_item_2 = dtft_item_k(&seq, 2);
    let output_dtft_item_3 = dtft_item_k(&seq, 3);
    let output_dtft_item_4 = dtft_item_k(&seq, 4);
    let output_dtft_item_5 = dtft_item_k(&seq, 5);
    let output_dtft_item_6 = dtft_item_k(&seq, 6);
    let output_dtft_item_7 = dtft_item_k(&seq, 7);

    let output_dtft = dtft(&seq);

    println!(
        "Output of dtft items in series: {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}, {:#.2?}",
        output_dtft_item_0, output_dtft_item_1, output_dtft_item_2, output_dtft_item_3,output_dtft_item_4, output_dtft_item_5, output_dtft_item_6, output_dtft_item_7,
    );
    println!("Output dtft series: {:#.2?}", output_dtft);

}
