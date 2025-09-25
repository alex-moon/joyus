# joyus

Simple Axum + SSE demo app with two components:
- A form with a single textarea where the user submits free text.
- A card which shows the last text submitted, updated live via Server-Sent Events (SSE).

Run locally:
1. Ensure Rust toolchain is installed.
2. Start the server:
   cargo run
3. Open http://127.0.0.1:3000 in your browser.

Notes:
- No frontend build step is required. The page uses a tiny inline script and native SSE.
- Multiple browser tabs will all update in real time when any tab submits text.
