# Deploying to Vercel

This project can be deployed to Vercel as a static site. The WASM files are already built and included in the `pkg/` directory.

## Quick Deploy

### Option 1: Using Vercel CLI (Recommended)

1. Install Vercel CLI (if not already installed):
   ```bash
   npm i -g vercel
   ```

2. Deploy:
   ```bash
   vercel
   ```

3. Follow the prompts to set up your project.

### Option 2: Using Vercel Dashboard

1. Push your code to GitHub/GitLab/Bitbucket
2. Go to [vercel.com](https://vercel.com)
3. Click "New Project"
4. Import your repository
5. Vercel will automatically detect it as a static site
6. Click "Deploy"

## Important Notes

- The `pkg/` directory contains the compiled WASM files and **must be included** in your repository for Vercel deployment
- The `vercel.json` file configures proper headers for WASM files (CORS and content types)
- The site will be available at `https://your-project.vercel.app`

## Rebuilding WASM Files

If you need to rebuild the WASM files:

1. Make sure you have Rust and wasm-bindgen installed:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

2. Build the WASM module:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   wasm-bindgen --target web --out-dir ./pkg ./target/wasm32-unknown-unknown/release/rs_wasm_experiment.wasm
   ```

3. Commit the updated `pkg/` directory to git

## Custom Domain

To use a custom domain:
1. Go to your project settings in Vercel dashboard
2. Navigate to "Domains"
3. Add your custom domain
4. Follow the DNS configuration instructions

