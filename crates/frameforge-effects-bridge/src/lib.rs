//! Thin integration boundary over EffectCraft's real effects registry.
//!
//! UI and timeline crates consume these categories without maintaining a duplicate catalog.

/// Effect categories supplied by the pinned EffectCraft revision.
pub fn categories() -> &'static [&'static str] {
    effectcraft_effects::CATEGORIES
}

#[cfg(test)]
mod tests {
    #[test]
    fn upstream_registry_exposes_color_correction() {
        assert!(super::categories().contains(&"Color Correction"));
    }
}
