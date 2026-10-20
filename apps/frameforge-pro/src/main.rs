//! FrameForge Pro application entry point.
//!
//! This first executable establishes the unified engine boundary.  A native egui workspace is
//! introduced after the renderer adapter is proven in CI, avoiding a UI that advertises
//! operations the engine cannot yet execute.

fn main() {
    let project = frameforge_core::Project::new("Untitled", frameforge_core::SequenceSettings::default());
    println!("FrameForge Pro {} — {} effect categories available", env!("CARGO_PKG_VERSION"), frameforge_effects_bridge::categories().len());
    println!("New project: {} ({}x{})", project.name, project.settings.width, project.settings.height);
}
