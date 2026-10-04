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

    let (mut found, mut missing, mut dangling) = (0, 0, 0);
    // Steam returns paths with mixed `\\` and `/` separators on non-Windows platforms, which
    // `steamworks` normalises. This counter is a regression check: it should stay at 0.
    let mut fixed = 0;
    let mut tally = |path: &Option<String>| match path {
        Some(path) if Path::new(path).exists() => found += 1,
        Some(path) => {
            dangling += 1;
            if Path::new(&path.replace('\\', "/")).exists() {
                fixed += 1;
            }
        }
        None => missing += 1,
    };

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
                    let png = input.get_glyph_png_for_action_origin(origin, size, style);
                    tally(&png);
                    match png {
                        Some(path) => println!(
                            "png {label} {size_name}: {path} (exists: {})",
                            Path::new(&path).exists()
                        ),
                        None => println!("png {label} {size_name}: None"),
                    }
                }
                let svg = input.get_glyph_svg_for_action_origin(origin, style);
                tally(&svg);
                match svg {
                    Some(path) => println!(
                        "svg {label}: {path} (exists: {})",
                        Path::new(&path).exists()
                    ),
                    None => println!("svg {label}: None"),
                }
            }
        }
    }
    println!(
        "\nfiles found: {found}, returned None: {missing}, path returned but missing on disk: \
         {dangling} (of which exist after replacing backslashes: {fixed})"
    );
    exit.write(AppExit::Success);
}

fn main() {
    // Use the demo Steam AppId for SpaceWar
    let steam = match SteamworksPlugin::init_app(480) {
        Ok(steam) => steam,
        Err(err) => {
            eprintln!("Failed to initialize Steam: {err}");
            eprintln!("Make sure the Steam client is running and logged in.");
            std::process::exit(1);
        }
    };
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(steam)
        .add_systems(Startup, print_glyphs)
        .run();
}
