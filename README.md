<p align="center">
  <img src="./assets/lib/icon.png" width="200" alt="Raycoon Logo" style="margin-bottom: 4px;" />
  <br />
  <img src="./assets/lib/title.png" width="250" alt="Raycoon Text" />
</p>

<br />

<p align="center" style="font-size: 25px;">
    A minimal <b>raycasting engine</b>  written in Rust.
</div>

<br />
<br />

Raycoon is a small, modern, and educational **2.5D raycasting engine** inspired by classic FPS techniques.
It focuses on clarity, simplicity, and clean architecture: the core provides pure logic (raycasting, collisions, tile mapping), while rendering and input are handled externally.

> [!NOTE]
>
> - Developed in personal free time.
> - Educational side project.
> - Contributions welcome.

## ✨ Features
- Minimal DDA-based raycasting engine  
- Tile-based world with configurable blocking tiles  
- Per-axis collision handling  
- Strict separation between **engine** and **renderer**  
- Optional **Macroquad renderer** provided as an example  
- Lightweight, hackable, and easy to understand

## 📦 Installation
Add the crate to your project:

```toml
[dependencies]
raycoon = "x.x.x"
```

The Macroquad renderer is behind a feature flag.
Enable it only if you need the provided backend:

```toml
[dependencies]
raycoon = { version = "x.x.x", features = ["macroquad-renderer"] }
```

Run the bundled example (requires the feature):

```bash
cargo run --example basic --features macroquad-renderer
```

## 🤝 Contributing
Contributions are welcome!  
Please keep the engine strictly backend-agnostic.  
Rendering, input handling, and tooling must remain in external modules.

## 📜 License
MIT License.
