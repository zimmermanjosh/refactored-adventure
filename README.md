# Flappy Game (Rust)
A 2D game project written in Rust, following the *Hands-On Rust* book. This project uses `bracket-lib` for rendering and game mechanics, and adopts the `xtask` pattern for customizable project automation.

---

## 📁 Project Structure

This project is organized as a Cargo workspace with the following structure:

```
├── Cargo.lock
├── Cargo.toml               # Workspace manifest
├── README.md
├── flappy_game/            # Main game crate
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── target/                 # Auto-generated build output
│   ├── CACHEDIR.TAG
│   └── debug/
│       ├── build/
│       ├── deps/
│       ├── examples/
│       └── incremental/
└── xtask/                  # Custom automation crate
    ├── Cargo.toml
    └── src/
        └── main.rs

11 directories, 8 files
```

---

## 🧱 Crates

### `flappy_game`
- Main game logic lives here.
- Uses [`bracket-lib`](https://crates.io/crates/bracket-lib) for terminal-based game rendering.

### `xtask`
- A custom binary crate for scripting dev tasks.
- Currently a placeholder; extend it to automate builds, asset copying, etc.

---

## 🛠 Usage

### Build the game
```bash
cargo build -p flappy_game
```

### Run the game
```bash
cargo run -p flappy_game
```

### Run a task from `xtask`
```bash
cargo run -p xtask -- help
```

---

## 📘 Learning Reference

> **Book:** *Hands-On Rust: Effective Learning Through 2D Game Development*  
> **Author:** Herbert Wolverson  
> [https://pragprog.com/titles/hwrust/hands-on-rust](https://pragprog.com/titles/hwrust/hands-on-rust)

---

This project is under development as a learning tool. Future goals include expanding `xtask` automation, adding game levels, and integrating unit tests.

