//! Rust bindings for macOS Core Animation with ergonomic builder APIs.
//!
//! # Builders
//!
//! | Builder | Purpose |
//! |---------|---------|
//! | [`WindowBuilder`] | Layer-backed windows with background, border, transparency |
//! | [`CALayerBuilder`] | Base layers with bounds, position, sublayers |
//! | [`CAShapeLayerBuilder`] | Vector shapes with path, fill, stroke, shadows |
//! | [`CATextLayerBuilder`] | Text rendering layers |
//! | [`CAEmitterLayerBuilder`](particles::CAEmitterLayerBuilder) | Particle systems with closure-based cell configuration |
//! | [`PointBurstBuilder`](particles::PointBurstBuilder) | Convenience API for radial particle bursts |
//! | [`CABasicAnimationBuilder`](animation_builder::CABasicAnimationBuilder) | Standalone GPU-accelerated animations |
//!
//! # Quick Start
//!
//! ```ignore
//! use core_animation::prelude::*;
//!
//! let window = WindowBuilder::new()
//!     .title("Demo")
//!     .size(400.0, 400.0)
//!     .centered()
//!     .background_color(Color::rgb(0.1, 0.1, 0.15))
//!     .build();
//!
//! let circle = CAShapeLayerBuilder::new()
//!     .circle(80.0)
//!     .position(CGPoint::new(200.0, 200.0))
//!     .fill_color(Color::CYAN)
//!     .animate("pulse", KeyPath::TransformScale, |a| {
//!         a.values(0.85, 1.15)
//!             .duration(1.seconds())
//!             .easing(Easing::InOut)
//!             .autoreverses()
//!             .repeat(Repeat::Forever)
//!     })
//!     .build();
//!
//! window.container().add_sublayer(&circle);
//! window.show_for(10.seconds());
//! ```
//!
//! # Animations
//!
//! All layer builders support `.animate()` for GPU-accelerated animations:
//!
//! ```ignore
//! .animate("name", KeyPath::TransformScale, |a| {
//!     a.values(0.8, 1.2)              // from/to values
//!         .duration(500.millis())      // timing
//!         .easing(Easing::InOut)       // curve
//!         .autoreverses()              // ping-pong
//!         .repeat(Repeat::Forever)     // loop
//!         .phase_offset(0.5)           // stagger multiple animations
//! })
//! ```
//!
//! **Animatable properties:** [`TransformScale`](animation_builder::KeyPath::TransformScale),
//! [`TransformRotation`](animation_builder::KeyPath::TransformRotation),
//! [`Opacity`](animation_builder::KeyPath::Opacity),
//! [`ShadowRadius`](animation_builder::KeyPath::ShadowRadius),
//! [`ShadowOpacity`](animation_builder::KeyPath::ShadowOpacity),
//! [`Custom`](animation_builder::KeyPath::Custom)
//!
//! **Easing curves:** [`Linear`](animation_builder::Easing::Linear),
//! [`In`](animation_builder::Easing::In),
//! [`Out`](animation_builder::Easing::Out),
//! [`InOut`](animation_builder::Easing::InOut)
//!
//! # Particle Systems
//!
//! ```ignore
//! use std::f64::consts::PI;
//!
//! let emitter = CAEmitterLayerBuilder::new()
//!     .position(320.0, 240.0)
//!     .shape(EmitterShape::Point)
//!     .particle(|p| {
//!         p.birth_rate(100.0)
//!             .lifetime(5.0)
//!             .velocity(80.0)
//!             .emission_range(PI * 2.0)
//!             .color(Color::CYAN)
//!             .image(ParticleImage::soft_glow(64))
//!     })
//!     .build();
//! ```
//!
//! Or use the convenience builder:
//!
//! ```ignore
//! let burst = PointBurstBuilder::new(320.0, 240.0)
//!     .velocity(100.0)
//!     .color(Color::PINK)
//!     .build();
//! ```
//!
//! **Particle images:** [`soft_glow`](particles::ParticleImage::soft_glow),
//! [`circle`](particles::ParticleImage::circle),
//! [`star`](particles::ParticleImage::star),
//! [`spark`](particles::ParticleImage::spark)
//!
//! # Examples
//!
//! See the [examples](https://github.com/sassman/core-animation-rs/tree/main/examples)
//! for runnable demos with screenshots.
//!
//! ```bash
//! cargo run --example window_builder
//! ```
//!
//! Use [`prelude`] to import common types.

#[cfg(all(not(docsrs), not(any(target_vendor = "apple"))))]
compile_error!("`core-animation` only works on Apple platforms. Pass `--target aarch64-apple-darwin` or similar to compile for macOS.");

pub mod animation_builder;
mod color;
mod duration_ext;
mod layer_builder;
mod layer_ext;
pub mod particles;
mod path_builder;
mod shape_layer_builder;
mod text_layer_builder;
pub mod window;

// Re-export Color type
pub use color::Color;

// Re-export the main types from objc2-quartz-core
pub use objc2_quartz_core::{CALayer, CAShapeLayer, CATextLayer, CATransform3D};

// Re-export our builders
pub use layer_builder::CALayerBuilder;
pub use path_builder::CGPathBuilder;
pub use shape_layer_builder::CAShapeLayerBuilder;
pub use text_layer_builder::{CATextLayerBuilder, TextAlign, Truncation};

// Re-export window types
pub use window::{Screen, Window, WindowBuilder, WindowLevel, WindowStyle};

// Re-export duration extension
pub use duration_ext::DurationExt;

// Re-export layer extension
pub use layer_ext::CALayerExt;

// Re-export dependencies for convenience
pub use objc2_core_foundation;
pub use objc2_core_graphics;
pub use objc2_core_text;
pub use objc2_quartz_core;

/// Prelude module for convenient imports.
pub mod prelude {
    // Color type
    pub use crate::color::Color;

    // Animation builder types
    pub use crate::animation_builder::{CABasicAnimationBuilder, Easing, KeyPath, Repeat};

    // Builders
    pub use crate::layer_builder::CALayerBuilder;
    pub use crate::particles::{
        CAEmitterCellBuilder, CAEmitterLayerBuilder, EmitterMode, EmitterShape, ParticleImage,
        PointBurstBuilder, RenderMode,
    };
    pub use crate::path_builder::CGPathBuilder;
    pub use crate::shape_layer_builder::CAShapeLayerBuilder;
    pub use crate::text_layer_builder::{CATextLayerBuilder, TextAlign, Truncation};
    pub use crate::window::{Screen, Window, WindowBuilder, WindowLevel, WindowStyle};

    // Duration extension for ergonomic timing
    pub use crate::duration_ext::DurationExt;

    // Layer extension for snake_case methods
    pub use crate::layer_ext::CALayerExt;

    // Core Animation types
    pub use crate::{CALayer, CAShapeLayer, CATextLayer, CATransform3D};
    pub use objc2_quartz_core::CABasicAnimation;

    // Core Foundation types (geometry, strings, collections, run loop)
    pub use objc2_core_foundation::{
        kCFRunLoopDefaultMode, kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks,
        CFAttributedString, CFDictionary, CFDictionaryKeyCallBacks, CFDictionaryValueCallBacks,
        CFIndex, CFRetained, CFRunLoop, CFString, CFStringBuiltInEncodings, CFTimeInterval,
        CGAffineTransform, CGFloat, CGPoint, CGRect, CGSize,
    };

    // Core Graphics types (context, colors, paths, transforms, display)
    pub use objc2_core_graphics::{
        CGAffineTransformIdentity, CGColor, CGContext, CGDirectDisplayID, CGDisplayBounds,
        CGMainDisplayID, CGPath,
    };

    // Core Text types (fonts, lines, string attributes)
    pub use objc2_core_text::{
        kCTFontAttributeName, kCTForegroundColorAttributeName, CTFont, CTLine,
    };

    // AppKit types (NSApplication)
    pub use objc2_app_kit::NSApplication;

    // Smart pointer for Objective-C objects
    pub use objc2::rc::Retained;
}
