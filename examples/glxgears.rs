//! Classic glxgears animation using Core Animation
//!
//! Recreates the iconic glxgears demo with three interlocking gears.
//!
//! Run with: `cargo run --example glxgears`

#[cfg(target_os = "macos")]
use core_animation::prelude::*;
#[cfg(target_os = "macos")]
use std::f64::consts::PI;

#[cfg(target_os = "macos")]
/// Draws a gear shape into the path builder.
fn draw_gear(
    p: CGPathBuilder,
    inner_radius: f64,
    outer_radius: f64,
    num_teeth: usize,
    tooth_depth: f64,
) -> CGPathBuilder {
    let r0 = inner_radius;
    let r1 = outer_radius - tooth_depth / 2.0;
    let r2 = outer_radius + tooth_depth / 2.0;
    let da = 2.0 * PI / num_teeth as f64 / 4.0;

    let mut p = p.move_to(r1, 0.0);

    for i in 0..num_teeth {
        let angle = (i as f64) * 2.0 * PI / num_teeth as f64;

        // Tooth profile: root -> tip -> tip -> root -> gap
        p = p
            .line_to(r2 * (angle + da).cos(), r2 * (angle + da).sin())
            .line_to(r2 * (angle + 2.0 * da).cos(), r2 * (angle + 2.0 * da).sin())
            .line_to(r1 * (angle + 3.0 * da).cos(), r1 * (angle + 3.0 * da).sin());

        let next_angle = ((i + 1) as f64) * 2.0 * PI / num_teeth as f64;
        p = p.line_to(r1 * next_angle.cos(), r1 * next_angle.sin());
    }

    p = p.close();

    // Center hole (opposite winding for fill rule)
    if r0 > 0.0 {
        p = p
            .move_to(r0, 0.0)
            .arc(0.0, 0.0, r0, 0.0, -2.0 * PI, true)
            .close();
    }

    p
}

#[cfg(target_os = "macos")]
#[allow(clippy::too_many_arguments)]
/// Creates a gear layer with rotation animation.
fn gear(
    inner: f64,
    outer: f64,
    teeth: usize,
    tooth_depth: f64,
    x: f64,
    y: f64,
    fill: Color,
    stroke: Color,
    duration: f64,
    clockwise: bool,
    phase: f64,
) -> Retained<CAShapeLayer> {
    let tip_radius = outer + tooth_depth / 2.0;
    let size = tip_radius * 2.0;

    CAShapeLayerBuilder::new()
        .draw_path(|p| draw_gear(p, inner, outer, teeth, tooth_depth))
        .bounds(CGRect::new(
            CGPoint::new(-tip_radius, -tip_radius),
            CGSize::new(size, size),
        ))
        .position(CGPoint::new(x, y))
        .fill_color(fill)
        .stroke_color(stroke)
        .stroke_width(1.0)
        .shadow(
            8.0,
            CGPoint::new(3.0, -3.0),
            Color::rgba(0.0, 0.0, 0.0, 0.5),
        )
        .animate("spin", KeyPath::TransformRotation, |a| {
            let dir = if clockwise { 1.0 } else { -1.0 };
            a.values(phase, phase + dir * 2.0 * PI)
                .duration(duration.seconds())
                .easing(Easing::Linear)
                .repeat(Repeat::Forever)
        })
        .build()
}

#[cfg(target_os = "macos")]
fn main() {
    let (width, height) = (640.0, 480.0);
    let scale = 28.0;

    let window = WindowBuilder::new()
        .title("glxgears")
        .size(width, height)
        .centered()
        .background_color(Color::rgb(0.0, 0.0, 0.0))
        .build();

    // Original glxgears timing: 70°/sec → 360°/70 ≈ 5.14s per rotation
    let base_duration = 360.0 / 70.0;
    let small_duration = base_duration / 2.0; // 2:1 gear ratio

    let (cx, cy) = (width / 2.0, height / 2.0);

    // Red gear (large, 20 teeth)
    let red = gear(
        1.0 * scale,
        4.0 * scale,
        20,
        0.7 * scale,
        cx - 3.0 * scale,
        cy + 2.0 * scale,
        Color::rgb(0.8, 0.1, 0.0),
        Color::rgb(0.5, 0.05, 0.0),
        base_duration,
        true,
        0.0,
    );

    // Green gear (small, 10 teeth, meshes with red)
    let green = gear(
        0.5 * scale,
        2.0 * scale,
        10,
        0.7 * scale,
        cx + 3.1 * scale,
        cy + 2.0 * scale,
        Color::rgb(0.0, 0.8, 0.2),
        Color::rgb(0.0, 0.5, 0.1),
        small_duration,
        false,
        -9.0_f64.to_radians(),
    );

    // Blue gear (small, 10 teeth, meshes with red)
    let blue = gear(
        1.3 * scale,
        2.0 * scale,
        10,
        0.7 * scale,
        cx - 3.1 * scale,
        cy - 4.2 * scale,
        Color::rgb(0.2, 0.2, 1.0),
        Color::rgb(0.1, 0.1, 0.6),
        small_duration,
        false,
        -25.0_f64.to_radians(),
    );

    // Title
    let title = CATextLayerBuilder::new()
        .text("glxgears")
        .font_size(18.0)
        .position(CGPoint::new(cx, 25.0))
        .foreground_color(Color::rgba(1.0, 1.0, 1.0, 0.8))
        .build();

    // Assemble scene
    let container = window.container();
    container.add_sublayer(&blue);
    container.add_sublayer(&green);
    container.add_sublayer(&red);
    container.add_sublayer(&title);

    println!("glxgears - Core Animation Edition");
    println!("Press Cmd+Q or close window to exit");

    window.show_for(15.seconds());
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This example only runs on macOS");
}
