//! Prints the glyph file Steam returns for a few Steam Input action origins in every
//! size and style, and checks that the files exist.
//!
//! Requires a running, logged-in Steam client. A controller is not expected to be needed
//! for fixed origins, but this example is how to find out.
//!
//! Run with: `cargo run --example glyphs --features raw-bindings`

use std::path::Path;

use bevy::{app::AppExit, prelude::*};
use bevy_steamworks::{
    sys::EInputActionOrigin, Client, InputGlyphBaseStyle, InputGlyphSize, InputGlyphStyle,
    SteamworksPlugin,
};

fn print_glyphs(client: Res<Client>, mut exit: MessageWriter<AppExit>) {
    let input = client.input();
    println!("Input::init -> {}", input.init(false));

    let origins = [
        ("None", EInputActionOrigin::k_EInputActionOrigin_None),
        (
            "XBoxOne_A",
            EInputActionOrigin::k_EInputActionOrigin_XBoxOne_A,
        ),
        (
            "SteamDeck_A",
            EInputActionOrigin::k_EInputActionOrigin_SteamDeck_A,
        ),
    ];
    let sizes = [
        ("small", InputGlyphSize::Small),
        ("medium", InputGlyphSize::Medium),
        ("large", InputGlyphSize::Large),
    ];
    let bases = [
        ("knockout", InputGlyphBaseStyle::Knockout),
        ("light", InputGlyphBaseStyle::Light),
        ("dark", InputGlyphBaseStyle::Dark),
    ];

    for (origin_name, origin) in origins {
        println!("\n== {origin_name} ==");
        println!("legacy: {:?}", input.get_glyph_for_action_origin(origin));
        for (base_name, base) in bases {
            for (neutral, solid) in [(false, false), (true, false), (false, true), (true, true)] {
                let style = InputGlyphStyle::new(base)
                    .neutral_color_abxy(neutral)
                    .solid_abxy(solid);
                let label = format!("{base_name} neutral={neutral} solid={solid}");
                for (size_name, size) in sizes {
                    match input.get_glyph_png_for_action_origin(origin, size, style) {
                        Some(path) => println!(
                            "png {label} {size_name}: {path} (exists: {})",
                            Path::new(&path).exists()
                        ),
                        None => println!("png {label} {size_name}: None"),
                    }
                }
                match input.get_glyph_svg_for_action_origin(origin, style) {
                    Some(path) => println!(
                        "svg {label}: {path} (exists: {})",
                        Path::new(&path).exists()
                    ),
                    None => println!("svg {label}: None"),
                }
            }
        }
    }
    exit.write(AppExit::Success);
}

fn main() {
    // Use the demo Steam AppId for SpaceWar
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(SteamworksPlugin::init_app(480).unwrap())
        .add_systems(Startup, print_glyphs)
        .run();
}
