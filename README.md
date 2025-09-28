# joyus

Simple Axum + SSE demo app with two components:
- A form with a single textarea where the user submits free text.
- A card which shows the last text submitted, updated live via Server-Sent Events (SSE).

What's inside:
- Rust backend (Axum) with modular structure (state, handlers, router).
- Frontend assets split into HTML (public/index.html), TypeScript and SCSS (web/src), bundled by Webpack to public/assets.

Run locally:
1. Ensure Rust toolchain is installed.
2. Build frontend assets (requires Node.js 18+):
   cd web && npm install && npm run build
3. Start the server from the repo root:
   cargo run
4. Open http://127.0.0.1:3000 in your browser.

Notes:
- You can use `npm run dev` in the web/ directory to watch asset changes during development.
- Multiple browser tabs will all update in real time when any tab submits text.
