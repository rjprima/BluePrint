# BluePrint()

A visual software/system designer and mapper, built around granular, spec-driven design, with the goal of making AI-assisted engineering less probabilistic and less expensive.

## Why

Both "vibe coding" and traditional spec-driven development run into the same wall: natural language is too vague to fully specify a system, and AI is not well-suited to architecting entire codebases in a maintainable way. A single sprawling prompt either under-specifies the system (leaving the AI to guess, inconsistently) or grows so detailed that writing it is effectively the same work as writing the code.

BluePrint() is built on a different premise: **strict separation of concerns between human and AI**. Humans architect the system. They decide structure, boundaries, and abstractions. AI writes code, at the method level, from a spec designed to remove ambiguity rather than compress it.

Instead of one long prompt for an entire feature or app, BluePrint() lets you visually design a system down to individual methods, then generates a tightly scoped prompt for a short-lived AI agent to implement just that piece. This enforces strict abstraction boundaries and keeps the system understandable and maintainable as it grows, while also:

- Cutting token usage, since each generation is scoped to a method, not a system
- Preventing context bloat and the drift that comes with it
- Reducing surface area for AI-introduced security flaws

## Status

**Early implementation — architecture complete, core app shell in progress.**

This is a GUI-first application, so most of the current work isn't independently demoable yet. What's functional so far:

- File creation and file path persistence

In active development:
- File loading, manipulation, and deletion
- Visual canvas / mapping interface
- Spec format and parser
- AI-assisted code generation pipeline

## Tech Stack

- **Rust** — core application logic
- **Tauri** — desktop app shell/runtime
- **Next.js** — frontend UI

## Getting Started

```bash
npx tauri dev
```

## Roadmap

- [x] Core architecture design
- [x] File creation & path persistence
- [ ] File loading, editing, deletion
- [ ] Visual canvas for system mapping
- [ ] Granular spec definition format
- [ ] AI-assisted code generation integration
- [ ] Existing-system mapping & import
