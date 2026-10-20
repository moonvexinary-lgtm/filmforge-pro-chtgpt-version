# Upstream audit — 2026-10-19

| Source | Revision used | License | Verified facts |
|---|---|---|---|
| FilmCraft | `2c5ba7a72619935f0481f99d59409e0d1844b234` | MIT OR Apache-2.0 | Rust 2024, Rust 1.95, `Tick(i64)`, 254,016,000,000 ticks/sec, rational `FrameRate`; media, edit, render and export crates exist. |
| EffectCraft | `7af58f009e889d9d0376769bec18f1e404b8d63d` | MIT OR Apache-2.0 | Rust 2024, Rust 1.95; imports pinned FilmCraft media crates; exposes real effect categories and typed effect/property architecture. |

The bridge must not invent effect APIs. It currently calls `effectcraft_effects::CATEGORIES` directly. Full catalog ingestion will be generated from the pinned registry and `docs/effects.md`, with per-effect rendering tests before UI parity is claimed.

The prior C++/Qt/MLT FrameForgePro reference is GPL-3.0 and is not copied, ported, or linked by this project.
