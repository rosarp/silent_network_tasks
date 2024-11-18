# WebSocket WASM Demo

A demonstration project showing WebSocket communication using Rust compiled to WebAssembly (WASM) and TypeScript.


## Features

- WebSocket client implementation in Rust + WASM
- Real-time message sending and receiving from browser based UI


## Prerequisites

- Rust (with wasm32-unknown-unknown target)
- Bun
- wasm-pack


## Project Structure
Note: Mentions main files in the project

- `src/`: Typescript code for browser UI
- `src-wasm/`: Rust code for WASM WebSocket client
- `src-wasm/pkg/`: Generated WASM module
- `public/`: Static files for frontend
- `package.json`: npm package configuration with preconfigured scripts


## Setup

1. Install Rust dependencies:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

2. Install Bun:
```bash
npm install -g bun
```

3. Install dependencies:
```bash
bun i
```

4. Build WASM using wasm-pack:
```bash
bun wasm
```

5. Run the app:
```bash
bun dev
```

6. Run the web socket server:
```bash
bun ws-server
```
