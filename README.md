<div align="center">

# Tauri Network Sniffer

[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB?logo=tauri)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178C6?logo=typescript)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-000000?logo=rust)](https://www.rust-lang.org/)

A lightweight, real-time desktop packet analyzer and network bandwidth monitor.
Built with a high-performance Rust core utilizing Tauri, and a fluid, responsive user interface powered by TypeScript and modern frontend frameworks.

</div>

## Overview

This application bridges low-level system networking with a high-level visual dashboard. It provides insights into active network sockets, protocol distributions, and data throughput without compromising system performance.

Currently, the application runs in a **high-velocity Mock Mode** to simulate gigabit network traffic safely without requiring root/administrative permissions or `libpcap` drivers.

## ⚙️ Technical Architecture & Data Flow

To handle high-velocity network traffic without freezing the user interface, the application separates concerns into a decoupled Producer-Consumer architecture using multi-threading.

1. **The Core (Rust Multi-threading):** The backend spawns a dedicated `tokio` worker thread that mocks listening to a network interface.
2. **The Batching Buffer:** Capturing raw packets at Gigabit speeds generates thousands of events per second. Emitting an IPC event for every single packet would instantly crash the frontend UI thread. To solve this, the Rust backend implements a throttling/batching buffer that aggregates packet metadata every 100ms. Bandwidth throughput is also calculated purely on the Rust side and emitted every 1s.
3. **The IPC Bridge (Tauri):** The aggregated batch is pushed to the frontend via asynchronous Tauri events (`tauri::Emitter`).
4. **The UI Thread (TypeScript):** The frontend receives the lightweight JSON payload, updates a state management store, and renders the data using a virtualized list to maintain a constant 60 FPS, even with tens of thousands of rows.

## 🛠️ Tech Stack & Mainstream Libraries

### Backend (Rust)
- **Tauri (v2):** The core framework used to build the cross-platform desktop application shell, providing secure, memory-efficient JS-to-Rust binding.
- **Tokio:** The asynchronous runtime used to manage multi-threaded tasks, timers for the batching buffer, and non-blocking operations.
- **Serde & Fastrand:** For fast struct-to-JSON serialization and mock data generation.

### Frontend (TypeScript & UI)
- **Shadcn UI & Tailwind CSS v4:** For building a polished, dark-themed, professional dashboard UI that mimics enterprise-level DevOps tooling.
- **Recharts:** Utilized to render real-time, streaming Area charts representing upload/download speeds (Bytes/sec).
- **TanStack Virtual:** A critical UI optimization library. Virtualized rendering ensures that only the visible rows of the massive packet log are mounted in the DOM.

## Getting Started

### Prerequisites

- Node.js >= 18
- pnpm >= 9
- Rust >= 1.70

*(On Linux, you may need to install Tauri system dependencies such as `libglib2.0-dev`, `libwebkit2gtk-4.1-dev`, `librsvg2-dev`, and `patchelf`.)*

### Install Dependencies

```bash
pnpm install
```

### Development Mode

Run the app in development mode with hot-reloading:

```bash
pnpm tauri dev
```

### Build for Production

```bash
pnpm tauri build
```

## Usage

1. Open the application.
2. Select a mock network interface (e.g., `eth0`, `wlan0`) from the dropdown.
3. Click the **Start** button.
4. Watch the real-time bandwidth metrics populate the top chart, and observe the high-velocity packet log streaming into the virtualized table below.
5. Click **Stop** to pause the capture, and use the **Trash** icon to clear the in-memory buffer.

## License

MIT