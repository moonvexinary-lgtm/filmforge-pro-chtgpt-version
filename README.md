# FrameForge Pro

FrameForge Pro is a native Rust nonlinear-editing application under active development. It is an independent implementation and is not derived from the GPL FrameForgePro/Kdenlive source base.

## Current integration milestone

The first committed increment provides a headless, deterministic project model and a genuine compile-time bridge to the pinned EffectCraft effects crate. It uses FilmCraft's `Tick(i64)` clock and exact rational frame rates, saves the project model as JSON, persists an EffectCraft stable effect ID and keyframes, and tests the save/load path.

It does **not** yet claim UI playback, media decoding, real effect rendering, or video export. Those are the next integration milestones and will be marked implemented only after their CI-backed tests exist.

## Build policy

No Cargo compilation runs locally. GitHub Actions on `windows-2025` performs formatting, checking, tests, Clippy, release compilation, ZIP packaging, and artifact upload. Development artifacts are retained for seven days.

## Upstream attribution

- [FilmCraft](https://github.com/storytold/filmcraft), MIT OR Apache-2.0 — exact time model and planned NLE/media adapters.
- [EffectCraft](https://github.com/storytold/effectcraft), MIT OR Apache-2.0 — real effects registry bridge and planned compositing adapters.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the integration boundary and [docs/UPSTREAM_AUDIT.md](docs/UPSTREAM_AUDIT.md) for the pinned-source audit.
