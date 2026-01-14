# Core Animation Examples

Examples demonstrating the `core-animation` crate's builder APIs.

## Running Examples

```bash
cargo run --example <name>
```

## Generating Recordings

Recordings (GIF and MP4) can be automatically generated using the provided script:

```bash
./tools/update-example-screenshots.sh
```

The script applies a patch that adds recording support, runs all examples, and reverts the patch afterward.

**Prerequisites:**
- ImageMagick (`brew install imagemagick`)
- ffmpeg (`brew install ffmpeg`)

---

## Examples

### `basic_layers`

Basic layer example with animated shapes. Demonstrates CALayer and CAShapeLayer with the builder APIs and GPU-accelerated animations using `.animate()`.

[![basic_layers](screenshots/basic_layers.gif)](screenshots/basic_layers.mp4)

```bash
cargo run --example basic_layers
```

---

### `breathing_circle`

Breathing circle - a soft pulsing orb with coordinated scale and opacity. Demonstrates multiple animations on one layer with `phase_offset` coordination.

[![breathing_circle](screenshots/breathing_circle.gif)](screenshots/breathing_circle.mp4)

```bash
cargo run --example breathing_circle
```

---

### `emitter`

Particle emitter using `CAEmitterLayerBuilder` with the closure-based particle configuration.

[![emitter](screenshots/emitter.gif)](screenshots/emitter.mp4)

```bash
cargo run --example emitter
```

---

### `loading_spinner`

Loading spinner with smooth rotating indicator using linear easing. Demonstrates `TransformRotation` animation with `Easing::Linear` for constant rotational speed.

[![loading_spinner](screenshots/loading_spinner.gif)](screenshots/loading_spinner.mp4)

```bash
cargo run --example loading_spinner
```

---

### `neon_glow`

Neon glow - retro neon sign effect with pulsing shadows. Demonstrates shadow property animations (`ShadowRadius`, `ShadowOpacity`) to create a glowing neon effect.

[![neon_glow](screenshots/neon_glow.gif)](screenshots/neon_glow.mp4)

```bash
cargo run --example neon_glow
```

---

### `particle_images`

Showcases all `ParticleImage` types side by side:
- `soft_glow` - Radial gradient (top-left)
- `circle` - Solid circle (top-right)
- `star` - Multi-pointed star (bottom-left)
- `spark` - Elongated streak (bottom-right)

[![particle_images](screenshots/particle_images.gif)](screenshots/particle_images.mp4)

```bash
cargo run --example particle_images
```

---

### `point_burst`

Demonstrates `PointBurstBuilder` - a convenience API for the common pattern of particles bursting from a point in all directions.

[![point_burst](screenshots/point_burst.gif)](screenshots/point_burst.mp4)

```bash
cargo run --example point_burst
```

---

### `ripple_rings`

Ripple rings - water ripple effect with concentric expanding rings. Demonstrates:
- `KeyPath::TransformScale` for ring expansion
- `KeyPath::Opacity` for rings fading as they expand
- `KeyPath::Custom("lineWidth")` for pulsing stroke width
- `phase_offset` for staggered timing across multiple rings
- `Easing::Out` for realistic ripple physics

[![ripple_rings](screenshots/ripple_rings.gif)](screenshots/ripple_rings.mp4)

```bash
cargo run --example ripple_rings
```

---

### `staggered_dots`

Staggered dots - classic loading indicator with phase-offset animations. Demonstrates using `phase_offset` to create staggered timing across multiple elements.

[![staggered_dots](screenshots/staggered_dots.gif)](screenshots/staggered_dots.mp4)

```bash
cargo run --example staggered_dots
```

---

### `window_builder`

Basic window creation with the `WindowBuilder` API.

[![window_builder](screenshots/window_builder.gif)](screenshots/window_builder.mp4)

```bash
cargo run --example window_builder
```
