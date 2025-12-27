use rustfft::{FftPlanner, num_complex::Complex};
use wasm_bindgen::prelude::*;

/// Computes the FFT of a time series data array
/// 
/// # Arguments
/// * `time_series` - A vector of f32 values representing the time series data
/// 
/// # Returns
/// A vector of f32 values representing the FFT result, interleaved as [real0, imag0, real1, imag1, ...]
#[wasm_bindgen]
pub fn compute_fft(time_series: Vec<f32>) -> Vec<f32> {
    // Convert the time series to complex numbers (real part = input value, imaginary part = 0)
    let mut complex_input: Vec<Complex<f32>> = time_series
        .iter()
        .map(|&value| Complex { re: value, im: 0.0 })
        .collect();

    // Create an FFT planner and plan the FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(complex_input.len());

    // Perform the FFT in-place
    fft.process(&mut complex_input);

    // Convert the complex result back to an interleaved array [real, imag, real, imag, ...]
    complex_input
        .iter()
        .flat_map(|c| vec![c.re, c.im])
        .collect()
}

/// Computes the FFT magnitude spectrum of a time series data array
/// 
/// # Arguments
/// * `time_series` - A vector of f32 values representing the time series data
/// 
/// # Returns
/// A vector of f32 values representing the magnitude of each frequency bin
#[wasm_bindgen]
pub fn compute_fft_magnitude(time_series: Vec<f32>) -> Vec<f32> {
    let fft_result = compute_fft(time_series);
    
    // Extract magnitude from interleaved [real, imag] pairs
    fft_result
        .chunks(2)
        .map(|chunk| {
            let re = chunk[0];
            let im = chunk[1];
            (re * re + im * im).sqrt()
        })
        .collect()
}

/// Parses a text file content string into a time series array
/// Each line should contain a single numeric value
/// 
/// # Arguments
/// * `file_content` - String content of a text file with one number per line
/// 
/// # Returns
/// A vector of f32 values parsed from the file content, or empty vector on error
#[wasm_bindgen]
pub fn parse_time_series_from_string(file_content: &str) -> Vec<f32> {
    file_content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<f32>().ok()
            }
        })
        .collect()
}

/// Convenience function: Computes FFT magnitude from a file content string
/// Combines parsing and FFT computation in one step
/// 
/// # Arguments
/// * `file_content` - String content of a text file with one number per line
/// 
/// # Returns
/// A vector of f32 values representing the magnitude of each frequency bin
#[wasm_bindgen]
pub fn compute_fft_magnitude_from_string(file_content: &str) -> Vec<f32> {
    let time_series = parse_time_series_from_string(file_content);
    if time_series.is_empty() {
        return Vec::new();
    }
    compute_fft_magnitude(time_series)
}

