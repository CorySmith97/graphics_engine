

use sokol::{
    app::{self as sapp, Keycode},
    debugtext as stdx,
    gfx::{self as sg, Image},
    glue as sglue, log as slog,
};

use image::*;

use std::path::Path;
use std::time::{Duration, Instant};


pub fn debug_stats(start: Instant) {
    stdx::canvas(sapp::widthf(), sapp::heightf());
    stdx::origin(0.0, 0.1);
    stdx::home();
    stdx::font(0);

    let duration = start.elapsed();
    let frame_stats = format!("Elapsed us: {:.1}", duration.as_millis());
    stdx::puts(frame_stats.as_str());
    stdx::crlf();
    stdx::draw();
    stdx::color3b(0, 0, 0);
}
