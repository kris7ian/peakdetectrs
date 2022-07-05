use pyo3::prelude::*;

#[pyfunction]
fn detect_peaks(
    array: Vec<f64>,
    lookahead: usize,
) -> PyResult<(Vec<(usize, f64)>, Vec<(usize, f64)>)> {
    let mut max_peaks: Vec<(usize, f64)> = Vec::new();
    let mut min_peaks: Vec<(usize, f64)> = Vec::new();
    let mut rolling_minimum: f64 = f64::INFINITY;
    let mut rolling_maximum: f64 = f64::NEG_INFINITY;
    let _delta = 0;

    let end = array.len() - lookahead;

    for (index, y) in array[..end].iter().enumerate() {
        let is_too_early = index < lookahead;

        //let look_for_maximum = rolling_maximum != f64::INFINITY;
        //let look_for_minimum = rolling_minimum != f64::NEG_INFINITY;

        // println!("Is_too_early: {}", is_too_early);
        // println!("Looking for max: {}", look_for_maximum);
        // println!("Looking for min: {}", look_for_minimum);
        // println!("index: {}", index);
        // println!("max: {}", rolling_maximum);
        // println!("min: {}", rolling_minimum);
        // println!("y: {}", y);
        // println!("-----------");

        if y >= &rolling_maximum {
            rolling_maximum = y.clone();
            let is_effective_maximum = &(array[index + 1..index + 1 + lookahead]
                .iter()
                .copied()
                .reduce(f64::max)
                .unwrap())
                < y;
            if is_effective_maximum && !is_too_early {
                max_peaks.push((index, y.clone()));
                rolling_maximum = f64::INFINITY;
                rolling_minimum = f64::INFINITY;
            }
        }

        if y <= &rolling_minimum {
            rolling_minimum = y.clone();
            let is_effective_minimum = &array[index + 1..index + 1 + lookahead]
                .iter()
                .copied()
                .reduce(f64::min)
                .unwrap()
                > y;
            if is_effective_minimum && !is_too_early {
                min_peaks.push((index, y.clone()));
                rolling_maximum = f64::NEG_INFINITY;
                rolling_minimum = f64::NEG_INFINITY;
            }
        }
    }
    return Ok((max_peaks, min_peaks));
}

/// A Python module implemented in Rust.
#[pymodule]
fn peakdetectrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect_peaks, m)?)?;
    Ok(())
}
