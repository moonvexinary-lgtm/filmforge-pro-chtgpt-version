# Architecture

FrameForge Pro is one application: a headless project/timeline core, adapters for FilmCraft media/NLE services and EffectCraft compositing, and a native desktop workspace above them. The application does not launch either upstream program.

## Time and project model

The core imports `filmcraft_time::Tick` directly. A tick represents 1/254,016,000,000 second; sequence frame rates remain rational pairs. JSON stores tick values as integers, preventing float-based drift during edit, save/load, and render planning.

The project model contains assets, audio/video tracks, clips with source and timeline ranges, track state, effect instances, and animated parameters. It is UI-independent and tested in CI.

## Rendering boundary

The NLE timeline selects active clips at an exact `Tick`. The EffectCraft adapter maps each persisted stable `ec.*` effect ID and evaluated parameters into EffectCraft's property/effect model. Preview and export will call the same frame-render contract. The current registry bridge deliberately exposes only upstream categories until the lossless parameter mapper and real frame renderer have tests.

## Milestone sequence

1. Deterministic project persistence and upstream registry/time compile bridge (current).
2. Media probe/decode and one EffectCraft frame effect rendered from a timeline clip.
3. MP4 export and automated decoded-output validation.
4. Native egui workspace using these commands, then timeline tooling, audio, and expanded effect controls.
