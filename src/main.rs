// For local testing, we'll include the FFT code directly
// In WASM, these functions will be exposed via wasm-bindgen
use rustfft::{FftPlanner, num_complex::Complex};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::env;
use std::path::Path;

/// Reads time series data from a text file where each data point is on a new line
fn read_time_series_from_file(filename: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut time_series = Vec::new();
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        match trimmed.parse::<f32>() {
            Ok(value) => time_series.push(value),
            Err(e) => {
                eprintln!("Warning: Could not parse line {} as f32: '{}' ({})", line_num + 1, trimmed, e);
            }
        }
    }
    
    if time_series.is_empty() {
        return Err("File contains no valid data points".into());
    }
    
    Ok(time_series)
}

/// Writes FFT results to a text file, one value per line
fn write_fft_to_file(filename: &str, fft_data: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    
    for value in fft_data {
        writeln!(file, "{}", value)?;
    }
    
    Ok(())
}

fn compute_fft(time_series: Vec<f32>) -> Vec<f32> {
    let mut complex_input: Vec<Complex<f32>> = time_series
        .iter()
        .map(|&value| Complex { re: value, im: 0.0 })
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(complex_input.len());
    fft.process(&mut complex_input);

    complex_input
        .iter()
        .flat_map(|c| vec![c.re, c.im])
        .collect()
}

fn compute_fft_magnitude(time_series: Vec<f32>) -> Vec<f32> {
    let fft_result = compute_fft(time_series);
    
    fft_result
        .chunks(2)
        .map(|chunk| {
            let re = chunk[0];
            let im = chunk[1];
            (re * re + im * im).sqrt()
        })
        .collect()
}

fn main() {
    // Get filename from command line arguments or use default
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "data.txt".to_string());

    println!("Reading time series data from: {}", filename);
    
    // Read time series from file
    let time_series = match read_time_series_from_file(&filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            eprintln!("\nUsage: cargo run [filename]");
            eprintln!("Example: cargo run data.txt");
            eprintln!("\nThe file should contain one numeric value per line.");
            std::process::exit(1);
        }
    };

    println!("Successfully read {} data points", time_series.len());
    println!("First 10 samples: {:?}", &time_series[..10.min(time_series.len())]);
    if time_series.len() > 10 {
        println!("Last 10 samples: {:?}", &time_series[(time_series.len() - 10)..]);
    }

    // Compute FFT
    println!("\nComputing FFT...");
    let fft_result = compute_fft(time_series.clone());
    println!("FFT result: {} values (interleaved real/imag)", fft_result.len());
    println!("First 10 values: {:?}", &fft_result[..10.min(fft_result.len())]);

    // Compute FFT magnitude
    let magnitude = compute_fft_magnitude(time_series);
    println!("\nFFT magnitude: {} frequency bins", magnitude.len());
    println!("First 10 magnitudes: {:?}", &magnitude[..10.min(magnitude.len())]);
    
    // Generate output filename
    let output_filename = if let Some(input_stem) = Path::new(&filename).file_stem().and_then(|s| s.to_str()) {
        format!("{}_fft.txt", input_stem)
    } else {
        "fft_output.txt".to_string()
    };
    
    // Write FFT magnitude to file
    match write_fft_to_file(&output_filename, &magnitude) {
        Ok(_) => {
            println!("\nFFT magnitude results written to: {}", output_filename);
        }
        Err(e) => {
            eprintln!("\nError writing output file: {}", e);
        }
    }
    
    // Also write the full FFT result (interleaved real/imag) to a separate file
    let output_full_filename = if let Some(input_stem) = Path::new(&filename).file_stem().and_then(|s| s.to_str()) {
        format!("{}_fft_full.txt", input_stem)
    } else {
        "fft_output_full.txt".to_string()
    };
    
    match write_fft_to_file(&output_full_filename, &fft_result) {
        Ok(_) => {
            println!("Full FFT result (interleaved real/imag) written to: {}", output_full_filename);
        }
        Err(e) => {
            eprintln!("Error writing full FFT output file: {}", e);
        }
    }
    
    // Find the peak frequency (assuming a sample rate - this is just for display)
    if let Some(peak_idx) = magnitude
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
    {
        let peak_magnitude = magnitude[peak_idx];
        println!("\nPeak magnitude: {:.6} at frequency bin {}", peak_magnitude, peak_idx);
    }
}
