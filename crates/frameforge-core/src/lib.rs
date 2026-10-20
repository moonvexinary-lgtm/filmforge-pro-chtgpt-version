//! Deterministic, UI-independent project and timeline model for FrameForge Pro.
//!
//! This crate deliberately uses FilmCraft's exact media clock.  Rendering adapters live above
//! this model so a timeline edit never depends on the direction of preview scrubbing.

use filmcraft_time::{FrameRate, Tick};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub use filmcraft_time::{TICKS_PER_SECOND, TimeRange};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub format_version: u32,
    pub name: String,
    pub settings: SequenceSettings,
    pub assets: BTreeMap<AssetId, MediaAsset>,
    pub tracks: Vec<Track>,
}

impl Project {
    pub fn new(name: impl Into<String>, settings: SequenceSettings) -> Self {
        Self {
            format_version: 1,
            name: name.into(),
            settings,
            assets: BTreeMap::new(),
            tracks: Vec::new(),
        }
    }

    pub fn save_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn load_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn clip_at(&self, time: Tick) -> Vec<ClipRef<'_>> {
        self.tracks
            .iter()
            .enumerate()
            .filter_map(|(track_index, track)| {
                track
                    .clips
                    .iter()
                    .find(|clip| clip.timeline_range().contains(time))
                    .map(|clip| ClipRef { track_index, clip })
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceSettings {
    pub frame_rate: RationalFrameRate,
    pub width: u32,
    pub height: u32,
    pub audio_sample_rate: u32,
}

impl Default for SequenceSettings {
    fn default() -> Self {
        Self {
            frame_rate: RationalFrameRate::from(FrameRate::FPS_24),
            width: 1920,
            height: 1080,
            audio_sample_rate: 48_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RationalFrameRate {
    pub numerator: i64,
    pub denominator: i64,
}

impl From<FrameRate> for RationalFrameRate {
    fn from(value: FrameRate) -> Self {
        Self {
            numerator: value.num,
            denominator: value.den,
        }
    }
}

impl From<RationalFrameRate> for FrameRate {
    fn from(value: RationalFrameRate) -> Self {
        FrameRate::new(value.numerator, value.denominator)
    }
}

pub type AssetId = String;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: AssetId,
    pub source_path: String,
    pub duration: TickValue,
    pub proxy_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub locked: bool,
    pub visible: bool,
    pub muted: bool,
    pub solo: bool,
    pub clips: Vec<Clip>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackKind {
    Video,
    Audio,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub asset_id: AssetId,
    pub start: TickValue,
    pub source_in: TickValue,
    pub duration: TickValue,
    pub effects: Vec<EffectInstance>,
}

impl Clip {
    pub fn timeline_range(&self) -> TimeRange {
        TimeRange::new(self.start.into(), self.duration.into())
    }
}

pub struct ClipRef<'a> {
    pub track_index: usize,
    pub clip: &'a Clip,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TickValue(pub i64);

impl From<Tick> for TickValue {
    fn from(value: Tick) -> Self {
        Self(value.0)
    }
}
impl From<TickValue> for Tick {
    fn from(value: TickValue) -> Self {
        Tick(value.0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectInstance {
    /// Stable EffectCraft effect ID, e.g. `ec.color.exposure`.
    pub effect_id: String,
    pub enabled: bool,
    pub parameters: BTreeMap<String, AnimatedValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimatedValue {
    pub default: ScalarValue,
    pub keyframes: Vec<Keyframe>,
}

impl AnimatedValue {
    /// Deterministic step evaluation.  Curve interpolation is intentionally delegated to the
    /// EffectCraft adapter until its property semantics are mapped losslessly.
    pub fn evaluate(&self, time: Tick) -> ScalarValue {
        self.keyframes
            .iter()
            .rev()
            .find(|key| Tick::from(key.time) <= time)
            .map(|key| key.value.clone())
            .unwrap_or_else(|| self.default.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: TickValue,
    pub value: ScalarValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScalarValue {
    Number(f64),
    Boolean(bool),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_round_trip_preserves_effect_animation_and_exact_time() {
        let mut project = Project::new("integration proof", SequenceSettings::default());
        project.assets.insert(
            "asset-a".into(),
            MediaAsset {
                id: "asset-a".into(),
                source_path: "fixture.mp4".into(),
                duration: TickValue(TICKS_PER_SECOND),
                proxy_path: None,
            },
        );
        project.tracks.push(Track {
            id: "V1".into(),
            kind: TrackKind::Video,
            locked: false,
            visible: true,
            muted: false,
            solo: false,
            clips: vec![Clip {
                id: "clip-a".into(),
                asset_id: "asset-a".into(),
                start: TickValue::default(),
                source_in: TickValue::default(),
                duration: TickValue(TICKS_PER_SECOND),
                effects: vec![EffectInstance {
                    effect_id: "ec.color.exposure".into(),
                    enabled: true,
                    parameters: BTreeMap::from([(
                        "exposure".into(),
                        AnimatedValue {
                            default: ScalarValue::Number(0.0),
                            keyframes: vec![Keyframe {
                                time: TickValue(TICKS_PER_SECOND / 2),
                                value: ScalarValue::Number(1.0),
                            }],
                        },
                    )]),
                }],
            }],
        });
        let loaded = Project::load_json(&project.save_json().unwrap()).unwrap();
        assert_eq!(loaded, project);
        assert_eq!(
            loaded.tracks[0].clips[0].effects[0].parameters["exposure"]
                .evaluate(Tick(TICKS_PER_SECOND / 2)),
            ScalarValue::Number(1.0)
        );
    }
}
