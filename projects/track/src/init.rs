//! Application setup

use crate::errors::Suggestion;
use error_stack::{Report, fmt::ColorMode};
use owo_colors::OwoColorize;

pub fn error_reporting() {
    Report::set_color_mode(ColorMode::Color);
    Report::install_debug_hook::<Suggestion>(|value, ctx| {
        let msg = value.0;
        let body = format!("suggestion {msg}");
        match ctx.color_mode() {
            ColorMode::None => ctx.push_body(body),
            ColorMode::Color => ctx.push_body(body.cyan().to_string()),
            ColorMode::Emphasis => ctx.push_body(body.italic().to_string()),
        }
    })
}
