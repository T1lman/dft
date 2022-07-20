pub fn dft(inreal: &[f64], inimag: &[f64], outreal: &mut [f64], outimag: &mut [f64]) {
    let n: usize = inreal.len();
    for k in 0..n {
        let mut sumreal: f64 = 0.0;
        let mut sumimag: f64 = 0.0;
        for t in 0..n {           
            let angle: f64 = 2.0 * std::f64::consts::PI * (t as f64) * (k as f64) / (n as f64);
            sumreal += inreal[t] * angle.cos() + inimag[t] * angle.sin();
            sumimag += -inreal[t] * angle.sin() + inimag[t] * angle.cos();
        }
        outreal[k] = sumreal;
        outimag[k] = sumimag;
    }
}
