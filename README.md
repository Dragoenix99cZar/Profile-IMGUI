# My Egui Web App

A fast, interactive, and lightweight web application built in **Rust** using the [egui](https://github.com/emilk/egui) GUI library and compiled to WebAssembly via [Trunk](https://trunkrs.dev/).

## 🚀 Quick Start

### Prerequisites

You need the standard Rust toolchain installed, along with the WebAssembly compilation target and the **Trunk** build tool.

1. **Install Rust:** Ensure you have Rust installed (via [rustup](https://rustup.rs/)).
2. **Add WASM Target:**
   ```
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk:
    ```
    cargo install --locked trunk
    ```
# 🛠️ Build and Run Commands
### Development (Local Server)
To spin up a local development server with hot-reloading (the app will automatically rebuild when you save changes):

```
trunk serve
```

Open http://127.0.0.1:8080 in your browser once it compiles.

### Production Build
To build a highly optimized release version ready for deployment:

```
trunk build --release
```

The compiled assets will be generated inside the /dist directory.


# 🔬 Standard Cargo Commands (Native Testing)
If your project is structured to also run natively (as a desktop app), you can use standard Cargo commands:

Run Natively: 
```
cargo run
```

Run Natively (Release): 
```
cargo run --release
```

Test: 
```
cargo test
```

# 📂 Project Structure
src/ - Contains the Rust source code.

main.rs / lib.rs - Application entry points.

app.rs - Your core egui UI logic and state.

index.html - The HTML entry point required by Trunk to load your WASM application.

Trunk.toml - (Optional) Trunk configuration file for tweaking asset paths or proxy settings.

## 💡 Quick Reference Cheat Sheet

| Command | Tool | Purpose |
| :--- | :--- | :--- |
| `trunk serve` | Trunk | Runs a local development server with hot-reloading. |
| `trunk build --release`| Trunk | Compiles the app into optimized HTML/WASM for production. |
| `cargo run` | Cargo | Runs the application natively as a desktop app (if supported). |
| `cargo check` | Cargo | Quickly checks your Rust code for compilation errors without building. |