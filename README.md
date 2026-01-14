# core-animation

Rust bindings for macOS Core Animation with ergonomic builder APIs.

## Builders

| Builder | Purpose |
|---------|---------|
| `WindowBuilder` | Layer-backed windows with background, border, transparency |
| `CALayerBuilder` | Base layers with bounds, position, sublayers |
| `CAShapeLayerBuilder` | Vector shapes with path, fill, stroke, shadows |
| `CATextLayerBuilder` | Text rendering layers |
| `CAEmitterLayerBuilder` | Particle systems with closure-based cell configuration |
| `PointBurstBuilder` | Convenience API for radial particle bursts |
| `CABasicAnimationBuilder` | Standalone GPU-accelerated animations |

## Quick Start

```rust
use core_animation::prelude::*;

let window = WindowBuilder::new()
    .title("Demo")
    .size(400.0, 400.0)
    .centered()
    .background_color(Color::rgb(0.1, 0.1, 0.15))
    .build();

let circle = CAShapeLayerBuilder::new()
    .circle(80.0)
    .position(CGPoint::new(200.0, 200.0))
    .fill_color(Color::CYAN)
    .animate("pulse", KeyPath::TransformScale, |a| {
        a.values(0.85, 1.15)
            .duration(1.seconds())
            .easing(Easing::InOut)
            .autoreverses()
            .repeat(Repeat::Forever)
    })
    .build();

window.container().add_sublayer(&circle);
window.show_for(10.seconds());
```

## Animations

All layer builders support `.animate()` for GPU-accelerated animations:

```rust
.animate("name", KeyPath::TransformScale, |a| {
    a.values(0.8, 1.2)              // from/to values
        .duration(500.millis())      // timing
        .easing(Easing::InOut)       // curve
        .autoreverses()              // ping-pong
        .repeat(Repeat::Forever)     // loop
        .phase_offset(0.5)           // stagger multiple animations
})
```

**Animatable properties:** `TransformScale`, `TransformRotation`, `Opacity`, `ShadowRadius`, `ShadowOpacity`, `Custom("propertyName")`

**Easing curves:** `Linear`, `In`, `Out`, `InOut`

## Particle Systems

```rust
use std::f64::consts::PI;

let emitter = CAEmitterLayerBuilder::new()
    .position(320.0, 240.0)
    .shape(EmitterShape::Point)
    .particle(|p| {
        p.birth_rate(100.0)
            .lifetime(5.0)
            .velocity(80.0)
            .emission_range(PI * 2.0)
            .color(Color::CYAN)
            .image(ParticleImage::soft_glow(64))
    })
    .build();
```

Or use the convenience builder:

```rust
let burst = PointBurstBuilder::new(320.0, 240.0)
    .velocity(100.0)
    .color(Color::PINK)
    .build();
```

**Particle images:** `soft_glow`, `circle`, `star`, `spark`

## Examples

```bash
cargo run --example window_builder
```

See [examples/README.md](examples/README.md) for all examples with screenshots.

## Platform

macOS only.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
