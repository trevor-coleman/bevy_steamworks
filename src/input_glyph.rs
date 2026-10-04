//! Support for styled Steam Input glyphs via `ISteamInput::GetGlyphPNGForActionOrigin`.
//!
//! `steamworks::Input::get_glyph_for_action_origin` only wraps the legacy glyph call, which
//! always returns Steam's default style. This module exposes the newer call.

#![allow(unsafe_code)]

use std::ffi::CStr;

use steamworks_sys as sys;

use crate::Client;

/// The pixel size of a glyph image.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum InputGlyphSize {
    /// 32x32 pixels.
    Small,
    /// 128x128 pixels.
    #[default]
    Medium,
    /// 256x256 pixels.
    Large,
}

impl InputGlyphSize {
    fn to_sys(self) -> sys::ESteamInputGlyphSize {
        match self {
            Self::Small => sys::ESteamInputGlyphSize::k_ESteamInputGlyphSize_Small,
            Self::Medium => sys::ESteamInputGlyphSize::k_ESteamInputGlyphSize_Medium,
            Self::Large => sys::ESteamInputGlyphSize::k_ESteamInputGlyphSize_Large,
        }
    }
}

/// The base look of a glyph.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum InputGlyphBaseStyle {
    /// Glyphs with the button shape knocked out of a solid background.
    #[default]
    Knockout,
    /// Light glyphs, for use on dark backgrounds.
    Light,
    /// Dark glyphs, for use on light backgrounds.
    Dark,
}

/// The full style of a glyph: a base style plus optional face button modifiers.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct InputGlyphStyle {
    /// The base style.
    pub base: InputGlyphBaseStyle,
    /// Use neutral colored A/B/X/Y face buttons.
    pub neutral_color_abxy: bool,
    /// Use solid A/B/X/Y face buttons.
    pub solid_abxy: bool,
}

impl InputGlyphStyle {
    /// Creates a style with the given base and no face button modifiers.
    pub fn new(base: InputGlyphBaseStyle) -> Self {
        Self {
            base,
            ..Self::default()
        }
    }

    /// Sets whether neutral colored A/B/X/Y face buttons are used.
    pub fn with_neutral_color_abxy(mut self, enabled: bool) -> Self {
        self.neutral_color_abxy = enabled;
        self
    }

    /// Sets whether solid A/B/X/Y face buttons are used.
    pub fn with_solid_abxy(mut self, enabled: bool) -> Self {
        self.solid_abxy = enabled;
        self
    }

    fn to_flags(self) -> u32 {
        use sys::ESteamInputGlyphStyle as S;
        let mut flags = match self.base {
            InputGlyphBaseStyle::Knockout => S::ESteamInputGlyphStyle_Knockout as u32,
            InputGlyphBaseStyle::Light => S::ESteamInputGlyphStyle_Light as u32,
            InputGlyphBaseStyle::Dark => S::ESteamInputGlyphStyle_Dark as u32,
        };
        if self.neutral_color_abxy {
            flags |= S::ESteamInputGlyphStyle_NeutralColorABXY as u32;
        }
        if self.solid_abxy {
            flags |= S::ESteamInputGlyphStyle_SolidABXY as u32;
        }
        flags
    }
}

impl Client {
    /// Returns the path of a PNG glyph image for an input action origin, in the requested
    /// size and style. Wraps `ISteamInput::GetGlyphPNGForActionOrigin`.
    ///
    /// Returns `None` if Steam has no glyph for the origin.
    ///
    /// Steam Input must have been initialized (see [`steamworks::Input::init`]).
    pub fn input_glyph_png_for_action_origin(
        &self,
        origin: sys::EInputActionOrigin,
        size: InputGlyphSize,
        style: InputGlyphStyle,
    ) -> Option<String> {
        // The `Client` guarantees the Steam API is initialized. The returned string is
        // owned by Steam and is copied before returning.
        unsafe {
            let input = sys::SteamAPI_SteamInput_v006();
            if input.is_null() {
                return None;
            }
            let path = sys::SteamAPI_ISteamInput_GetGlyphPNGForActionOrigin(
                input,
                origin,
                size.to_sys(),
                style.to_flags(),
            );
            if path.is_null() {
                return None;
            }
            let path = CStr::from_ptr(path).to_string_lossy().into_owned();
            (!path.is_empty()).then_some(path)
        }
    }
}
