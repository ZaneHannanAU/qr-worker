mod utils;

use cfg_if::cfg_if;
use qrcode::{render::svg::Color, EcLevel, QrCode};
use smartstring::alias::String;
use std::str::Split;
use url::Url;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

struct Config {
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
    pub ec_level: EcLevel,
    pub bg: String,
    pub fg: String,
    pub quiet_zone: bool,
}

// https://example.com/fg=000000/bg=ffffff/min=128/max=256/ec=m/qz=1?data
fn parse_config(segments: &mut Split<char>) -> Config {
    let mut config = Config {
        min_size: None,
        max_size: None,
        ec_level: EcLevel::M,
        bg: String::from("#ffffff"),
        fg: String::from("#000000"),
        quiet_zone: true,
    };

    for e in segments {
        if let Some((k, v)) = e.split_once("=") {
            use EcLevel::*;
            match k {
                "min" => {
                    config.min_size = v.parse().ok();
                }
                "max" => {
                    config.max_size = v.parse().ok();
                }
                "ec" => {
                    config.ec_level = match v {
                        "l" => L,
                        "m" => M,
                        "q" => Q,
                        "h" => H,
                        _ => M,
                    };
                }
                "bg" if v.len() <= 10 => {
                    let mut s = String::from("#");
                    s.push_str(v);
                    config.bg = s;
                }
                "fg" if v.len() <= 10 => {
                    let mut s = String::from("#");
                    s.push_str(v);
                    config.fg = s;
                }
                "qz" => {
                    config.quiet_zone = match v {
                        "1" => true,
                        "0" => false,
                        "true" => true,
                        "false" => false,
                        "yes" => true,
                        "no" => false,
                        _ => continue,
                    };
                }
                _ => continue,
            }
        }
    }
    config
}

#[wasm_bindgen]
pub fn handle_request(url: &str) -> Result<::std::string::String, JsValue> {
    let url =
        Url::parse(&url).map_err(|e| JsValue::from_str(&format!("unable to parse url: {}", e)))?;

    let text = match url.query() {
        Some(text) => text.to_owned(),
        None => url.clone().into_string(),
    };

    let mut segments = url
        .path_segments()
        .ok_or(JsValue::from_str("cannot-be-a-base URL"))?;

    let cfg = parse_config(&mut segments);

    let code = QrCode::with_error_correction_level(text.as_bytes(), cfg.ec_level)
        .map_err(|e| JsValue::from_str(&format!("unable to create qr code: {}", e)))?;

    let mut image_builder = code.render();

    let mut image_builder = image_builder
        .dark_color(Color(&cfg.fg))
        .light_color(Color(&cfg.bg))
        .quiet_zone(cfg.quiet_zone);

    if let Some(size) = cfg.min_size {
        image_builder = image_builder.min_dimensions(size, size);
    }

    if let Some(size) = cfg.max_size {
        image_builder = image_builder.max_dimensions(size, size);
    }

    Ok(image_builder.build())
}
