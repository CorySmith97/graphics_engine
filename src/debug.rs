use sokol::{
    app::{self as sapp, Keycode},
    debugtext as stdx,
    gfx::{self as sg, Image},
};

use std::time::{Duration, Instant};

pub fn debug_stats(start: Instant) {
    stdx::canvas(sapp::widthf(), sapp::heightf());
    stdx::origin(0.0, 0.1);
    stdx::home();
    stdx::font(0);

    let duration = start.elapsed();
    let frame_stats = format!("Elapsed us: {:.1}", duration.as_micros());
    stdx::puts(frame_stats.as_str());
    stdx::crlf();
    stdx::draw();
    stdx::color3b(0, 0, 0);
}
