# Contributing to Raycoon

Thank you for your interest in contributing to **Raycoon**!  
This project aims to keep a clean, simple, and educational codebase.  
Please read the following guidelines before submitting changes.

## 🧱 Project Philosophy

Raycoon is built around a few core principles:

- **Engine-first**: the core must remain strictly backend-agnostic.  
- **No rendering or input in the engine**: all frontend logic belongs to external modules (e.g., Macroquad renderer).  
- **Readable and hackable code**: clarity is preferred over cleverness.  
- **Small, focused API**: avoid unnecessary abstractions or overengineering.  
- **Safe, idiomatic Rust** whenever possible.

If a change violates these principles, it will be discussed before merging.

## 🛠️ How to Contribute

### 1. Fork the repository
Create your own fork and clone it locally.

### 2. Create a feature branch

Use a clear branch name depending on the type of change:

- `feature/...` – for new features or improvements  
  - ex: `feature/add-rayhit-side`
- `bugfix/...` – for fixes to existing issues  
  - ex: `bugfix/collision-bounds`

Example:

```bash
git switch -c feature/my-change
```

### 3. Make your changes
Follow the guidelines below (Rust style, separation rules, testing).

### 4. Ensure the project builds and runs
```bash
cargo build
cargo --example basic
```

### 5. Open a Pull Request
Describe:
- what you changed  
- why you changed it  

Pull requests that include clear explanations are merged much faster.

## 🧼 Code Guidelines

- Keep the engine **backend-agnostic** (no rendering, no input).
- Write **simple, clear, and readable** Rust code.
- Prefer **safe, idiomatic Rust** (no `unsafe`, avoid unnecessary clones).
- Keep the API **small and focused**.
- Avoid over-engineering and unnecessary abstractions.
- Add tests when relevant (raycasting, collisions, map logic).

## 🧾 Commit Messages

Use clear, conventional commits:

- refactor(engine): simplify raycasting loop
- feat(core): add cast_view helper
- fix(collision): correct per-axis movement
- docs: update README

## 💬 Discussions & Issues

Before implementing large or breaking changes:
- open an issue  
- describe your idea  
- discuss it with maintainers  

We value discussion before implementation.

## 📜 License

By contributing to Raycoon, you agree that your contributions will be licensed under the **MIT License**, the same license used by the project.

Thank you again for helping improve Raycoon! 🦝