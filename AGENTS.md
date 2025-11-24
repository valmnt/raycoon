# AGENTS Guidelines

## Context
Raycoon is a lightweight **2.5D raycasting engine** written in Rust.  
Its goal is to provide:
- a clean, backend-agnostic core (pure engine logic),
- optional renderers (Macroquad now, others later),
- an educational, long-term side project for learning Rust and game engine architecture.

The **engine** must stay pure: no rendering, no input, no OS/GPU logic.  
Renderers are small adapters living behind feature flags.

## Coding Style
- **Rust 2021**, idiomatic and safe. Avoid `unsafe`.
- Keep the engine logic strict and explicit — no hidden side effects.
- **Separation of concerns**
  - Engine = math, world, rays, camera, hits.
  - Renderer = drawing, textures, window, inputs.
- Prefer **small, clear APIs** over deep abstractions.
- Use:
  - `snake_case` for files/functions/variables  
  - `CamelCase` for types  
  - `SCREAMING_SNAKE_CASE` for constants
- Document only what is non-obvious.
- Use `glam` inside the engine; convert types at renderer boundary.

## Commit Structure
Use **Conventional Commits**:

**Allowed types**
`feat`, `fix`, `refactor`, `style`, `docs`, `test`, `chore`, `perf`

**Examples**
- `feat: add ray hit side detection`
- `fix: correct FOV angle normalization`
- `refactor: split engine modules for clarity`
- `perf: reuse buffers to reduce allocations`
- `docs: explain map format and coordinates`

**Rules**
- English, present tense (“add”, not “added”)
- Clear, explicit messages
- If helpful, add a short “why” in the body

## Agent Guidelines
The agent must **help the developer learn**, not just provide answers.

- **Guide first, answer second**
  Provide explanation, reasoning, or direction before giving code.

- **Never dump full files**
  Prefer small snippets, diffs, or pseudo-code.

- **Encourage understanding**
  Explain ownership issues, math, architecture decisions, etc.

- **Respect project boundaries**
  - Don’t move rendering into the engine.  
  - Don’t introduce unnecessary abstractions.

- **Ask if unsure**
  When context is unclear, request clarification rather than guessing.