mod utils;

fn main() {
    let in1 = vec![10.0; 10];
    let in2 = vec![3.0; 10];
    let mut out1: Vec<f64> = vec![0.0; 10];
    let mut out2: Vec<f64> = vec![0.0; 10];
    utils::dft(&in1, &in2, &mut out1, &mut out2);
    println!("realpart: {:?}\nimaginary part:{:?}", out1, out2);
}
