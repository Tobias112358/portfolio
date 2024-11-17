# Rust WASM Web Portfolio

This project is a web-based portfolio built using Rust and WebAssembly (WASM).

## Overview

This portfolio showcases my skills and projects using a combination of Rust for the backend logic and WebAssembly for seamless integration with the web frontend.

## Features

- Fast and efficient performance thanks to Rust and WASM
- Interactive UI components
- Project showcase
- About me section
- Contact form

## Prerequisites

- Rust (latest stable version)
- wasm-pack
- Node.js and npm

## Getting Started

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-wasm-portfolio.git
   cd rust-wasm-portfolio
   ```

2. Build the Rust code to WebAssembly:
   ```
   wasm-pack build
   ```

3. Install JavaScript dependencies:
   ```
   cd www
   npm install
   ```

4. Start the development server:
   ```
   npm run start
   ```

5. Open your browser and navigate to `http://localhost:8080`

## Building for Production

To create a production build, run:


# Notes
Use the command:
```wasm-pack build --target web --out-dir ./www/pkg```
to build wasm from root.
