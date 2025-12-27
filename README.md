# Rust FFT to WebAssembly Experiment

A proof of concept project that demonstrates converting time series data arrays to FFT (Fast Fourier Transform) data arrays using Rust, compiled to WebAssembly.

## Features

- Fast FFT computation using the `rustfft` crate
- WASM-compatible functions exposed via `wasm-bindgen`
- Two main functions:
  - `compute_fft`: Returns FFT result as interleaved real/imaginary pairs
  - `compute_fft_magnitude`: Returns the magnitude spectrum (useful for frequency analysis)

## Input Format

The input data should be a text file where each data point is on a new line. For example:

```
0.0
1.0
0.0
-1.0
0.0
```

Empty lines are automatically skipped.

## Local Testing

To test the FFT functionality locally with a file:

```bash
cargo run [filename]
```

For example:
```bash
cargo run data.txt
```

If no filename is provided, it defaults to `data.txt`. A sample `data.txt` file is included in the repository.

The program will generate two output files:
- `{input_filename}_fft.txt`: Contains the FFT magnitude spectrum (one value per line)
- `{input_filename}_fft_full.txt`: Contains the full FFT result as interleaved real/imaginary pairs (one value per line)

For example, if you process `data.txt`, the output files will be:
- `data_fft.txt`: Magnitude spectrum
- `data_fft_full.txt`: Full FFT result (real/imag pairs)

## Deployment to Vercel

This project can be easily deployed to Vercel. See [DEPLOY.md](./DEPLOY.md) for detailed instructions.

Quick deploy:
```bash
npm i -g vercel
vercel
```

**Note:** The `pkg/` directory (containing WASM files) must be committed to your repository for Vercel deployment.

## Building for WebAssembly

### Prerequisites

1. Install the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Install `wasm-bindgen-cli`:
   ```bash
   cargo install wasm-bindgen-cli
   ```

### Build Steps

1. Build the project for WASM:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. Generate WASM bindings:
   ```bash
   wasm-bindgen --target web --out-dir ./pkg ./target/wasm32-unknown-unknown/release/rs_wasm_experiment.wasm
   ```

   The output will be in the `pkg/` directory.

## Interactive HTML Demo

An interactive HTML demo page is included! To run it:

1. Make sure you've built the WASM module (see "Building for WebAssembly" above)

2. Start a local web server. You can use the included helper script:
   ```bash
   ./start-server.sh
   ```
   
   Or manually start a server using one of these options:
   
   **Python 3:**
   ```bash
   python3 server.py
   ```
   
   **PHP:**
   ```bash
   php -S localhost:8000
   ```
   
   **Node.js (http-server):**
   ```bash
   npx http-server -p 8000
   ```

3. Open your browser to `http://localhost:8000/index.html`

The demo page allows you to:
- Upload a text file with time series data (drag & drop or click to browse)
- View the FFT magnitude spectrum and full FFT results
- Download the results as text files

**Note:** You must use a web server (not just open the HTML file directly) because WASM modules require proper CORS headers.

### Using in JavaScript

After building, you can use the WASM module in your web project:

**Option 1: Using an array directly**

```javascript
import init, { compute_fft, compute_fft_magnitude } from './pkg/rs_wasm_experiment.js';

async function run() {
  // Initialize the WASM module
  await init();

  // Example: Process a time series array
  const timeSeries = [0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0];
  
  // Get FFT result (interleaved real/imaginary pairs)
  const fftResult = compute_fft(timeSeries);
  
  // Get magnitude spectrum
  const magnitude = compute_fft_magnitude(timeSeries);
  
  console.log('FFT Result:', fftResult);
  console.log('Magnitude:', magnitude);
}

run();
```

**Option 2: Reading from a file (browser)**

```javascript
import init, { parse_time_series_from_string, compute_fft_magnitude_from_string } from './pkg/rs_wasm_experiment.js';

async function run() {
  await init();
  
  // Read file using File API
  const fileInput = document.getElementById('fileInput');
  const file = fileInput.files[0];
  
  if (file) {
    const fileContent = await file.text();
    
    // Parse and compute FFT magnitude in one step
    const magnitude = compute_fft_magnitude_from_string(fileContent);
    
    console.log('Magnitude spectrum:', magnitude);
  }
}

// Or parse first, then compute
async function processFile() {
  await init();
  
  const fileContent = await file.text();
  const timeSeries = parse_time_series_from_string(fileContent);
  const magnitude = compute_fft_magnitude(timeSeries);
  
  console.log('Parsed', timeSeries.length, 'data points');
  console.log('Magnitude spectrum:', magnitude);
}
```

### HTML Example with File Upload

You can create an HTML file that allows users to upload a text file and compute its FFT:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>FFT WASM Demo</title>
</head>
<body>
    <h1>FFT Analysis</h1>
    <input type="file" id="fileInput" accept=".txt">
    <button onclick="processFile()">Compute FFT</button>
    <div id="output"></div>
    
    <script type="module">
        import init, { compute_fft_magnitude_from_string } from './pkg/rs_wasm_experiment.js';
        
        let wasmReady = false;
        
        init().then(() => {
            wasmReady = true;
            console.log('WASM module loaded');
        });
        
        window.processFile = async function() {
            if (!wasmReady) {
                alert('WASM module not ready yet');
                return;
            }
            
            const fileInput = document.getElementById('fileInput');
            const file = fileInput.files[0];
            
            if (!file) {
                alert('Please select a file');
                return;
            }
            
            try {
                const fileContent = await file.text();
                const magnitude = compute_fft_magnitude_from_string(fileContent);
                
                const output = document.getElementById('output');
                output.innerHTML = `
                    <h2>Results</h2>
                    <p>Number of frequency bins: ${magnitude.length}</p>
                    <p>First 20 magnitudes: ${magnitude.slice(0, 20).map(m => m.toFixed(4)).join(', ')}</p>
                `;
                
                console.log('FFT magnitude spectrum:', magnitude);
            } catch (error) {
                console.error('Error processing file:', error);
                alert('Error processing file: ' + error.message);
            }
        };
    </script>
</body>
</html>
```

## API Reference

### `compute_fft(time_series: Vec<f32>) -> Vec<f32>`

Computes the FFT of a time series data array.

- **Input**: Array of f32 values representing the time series
- **Output**: Array of f32 values, interleaved as [real₀, imag₀, real₁, imag₁, ...]

### `compute_fft_magnitude(time_series: Vec<f32>) -> Vec<f32>`

Computes the magnitude spectrum of a time series data array.

- **Input**: Array of f32 values representing the time series
- **Output**: Array of f32 values representing the magnitude of each frequency bin

### `parse_time_series_from_string(file_content: &str) -> Vec<f32>`

Parses a text file content string into a time series array. Each line should contain a single numeric value. Empty lines are skipped.

- **Input**: String content of a text file with one number per line
- **Output**: Array of f32 values parsed from the file content

### `compute_fft_magnitude_from_string(file_content: &str) -> Vec<f32>`

Convenience function that combines parsing and FFT magnitude computation in one step.

- **Input**: String content of a text file with one number per line
- **Output**: Array of f32 values representing the magnitude of each frequency bin

## Dependencies

- `rustfft`: Fast FFT library for Rust
- `wasm-bindgen`: Rust and WebAssembly interop

## License

This is a proof of concept project.

