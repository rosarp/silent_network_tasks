# Cloud Sync

A real-time communication application built with Rust (Axum) and Alpine.js that enables users to connect and chat through WebSocket channels.

## Features

- Real-time WebSocket message sync
- Automatic timeout for inactive connections after 10 seconds without a second party
- No node dependency, simple html UI


## Prerequisites

- Rust with Axum web framework
- WebSocket for real-time communication
- Tokio for async runtime and MPSC channels


## Project Structure
Note: Mentions main files in the project

- `src/main.rs`: Server setup and configuration
- `src/handler.rs`: WebSocket handler implementation
- `assets/index.html`: Main UI
- `assets/websocket.js`: WebSocket client logic
- `assets/main.js`: Main Js initialization


## How It Works

1. Users connect by entering a channel ID
2. The application waits for a second party to join the same channel
3. Once connected, users can send and receive messages in real-time
4. Connection is automatically closed if:
   - No second party joins within 10 seconds
   - User manually disconnects
   - WebSocket connection is lost


## Getting Started

1. Clone the repository
2. Start the server:
   ```bash
   cargo run
   ```
3. Open http://localhost:3000 in your browser
4. Enter a channel ID and click Connect
5. Share the channel ID with another user to start chatting


## TODO:

1. Fix Boundary cases