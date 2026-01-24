//! Builder for `CGPath` (vector path construction).
//!
//! Provides an ergonomic API for constructing Core Graphics paths using
//! simple Rust types (`f64` for coordinates, radians for angles).
//!
//! # Basic Usage
//!
//! ```ignore
//! let path = CGPathBuilder::new()
//!     .move_to(0.0, 0.0)
//!     .line_to(100.0, 0.0)
//!     .line_to(100.0, 100.0)
//!     .line_to(0.0, 100.0)
//!     .close()
//!     .build();
//! ```
//!
//! # With Transforms
//!
//! A transform can be set that applies to all subsequent operations:
//!
//! ```ignore
//! let path = CGPathBuilder::new()
//!     .transform(CGAffineTransform::new_scale(2.0, 2.0))
//!     .move_to(0.0, 0.0)  // Will be scaled
//!     .line_to(50.0, 0.0) // Will be scaled
//!     .no_transform()
//!     .line_to(100.0, 100.0) // Not scaled
//!     .build();
//! ```
//!
//! # Drawing Shapes
//!
//! Convenience methods for common shapes:
//!
//! ```ignore
//! let path = CGPathBuilder::new()
//!     .circle(50.0, 50.0, 50.0)      // center_x, center_y, diameter
//!     .rect(0.0, 0.0, 100.0, 100.0)  // x, y, width, height
//!     .rounded_rect(10.0, 10.0, 80.0, 80.0, 5.0) // with corner radius
//!     .build();
//! ```

use objc2_core_foundation::{CFRetained, CGAffineTransform, CGPoint, CGRect, CGSize};
use objc2_core_graphics::{CGMutablePath, CGPath};

/// Builder for constructing `CGPath` instances.
///
/// Uses `CGMutablePath` internally and provides an ergonomic chainable API
/// with simple Rust types.
///
/// # Examples
///
/// ## Simple Triangle
///
/// ```ignore
/// let triangle = CGPathBuilder::new()
///     .move_to(50.0, 0.0)
///     .line_to(100.0, 100.0)
///     .line_to(0.0, 100.0)
///     .close()
///     .build();
/// ```
///
/// ## Gear Tooth (conceptual)
///
/// ```ignore
/// let gear = CGPathBuilder::new()
///     .move_to(outer_x, outer_y)
///     .arc(0.0, 0.0, outer_radius, start_angle, tooth_start, false)
///     .line_to(tip_x1, tip_y1)  // tooth rise
///     .line_to(tip_x2, tip_y2)  // tooth top
///     .line_to(next_x, next_y)  // tooth fall
///     // ... repeat for each tooth
///     .close()
///     .build();
/// ```
pub struct CGPathBuilder {
    path: CFRetained<CGMutablePath>,
    transform: Option<CGAffineTransform>,
}

impl Default for CGPathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CGPathBuilder {
    /// Creates a new path builder.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = CGPathBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: CGMutablePath::new(),
            transform: None,
        }
    }

    // ========================================================================
    // Transform management
    // ========================================================================

    /// Sets a transform to apply to all subsequent path operations.
    ///
    /// The transform remains active until changed or cleared with
    /// [`no_transform`](Self::no_transform).
    ///
    /// # Arguments
    ///
    /// * `transform` - The affine transform to apply
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let path = CGPathBuilder::new()
    ///     .transform(CGAffineTransform::new_scale(2.0, 2.0))
    ///     .move_to(10.0, 10.0)  // Becomes (20.0, 20.0)
    ///     .line_to(20.0, 20.0)  // Becomes (40.0, 40.0)
    ///     .build();
    /// ```
    #[must_use]
    pub fn transform(mut self, transform: CGAffineTransform) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Clears the current transform.
    ///
    /// Subsequent operations will not have any transform applied.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let path = CGPathBuilder::new()
    ///     .transform(some_transform)
    ///     .move_to(0.0, 0.0)    // Transformed
    ///     .no_transform()
    ///     .line_to(10.0, 10.0)  // Not transformed
    ///     .build();
    /// ```
    #[must_use]
    pub fn no_transform(mut self) -> Self {
        self.transform = None;
        self
    }

    /// Returns a pointer to the current transform, or null if none is set.
    fn transform_ptr(&self) -> *const CGAffineTransform {
        match &self.transform {
            Some(t) => t as *const CGAffineTransform,
            None => std::ptr::null(),
        }
    }

    // ========================================================================
    // Core path operations
    // ========================================================================

    /// Moves the current point to the specified coordinates.
    ///
    /// This begins a new subpath at the given point. Any subsequent
    /// line or curve operations will start from this point.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    ///
    /// # Examples
    ///
    /// ```ignore
    /// builder.move_to(100.0, 50.0)
    /// ```
    #[must_use]
    pub fn move_to(self, x: f64, y: f64) -> Self {
        unsafe {
            CGMutablePath::move_to_point(Some(&self.path), self.transform_ptr(), x, y);
        }
        self
    }

    /// Adds a straight line from the current point to the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the line endpoint
    /// * `y` - The y-coordinate of the line endpoint
    ///
    /// # Examples
    ///
    /// ```ignore
    /// builder
    ///     .move_to(0.0, 0.0)
    ///     .line_to(100.0, 100.0)
    /// ```
    #[must_use]
    pub fn line_to(self, x: f64, y: f64) -> Self {
        unsafe {
            CGMutablePath::add_line_to_point(Some(&self.path), self.transform_ptr(), x, y);
        }
        self
    }

    /// Closes the current subpath by adding a line from the current point
    /// to the starting point of the subpath.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Creates a closed triangle
    /// builder
    ///     .move_to(50.0, 0.0)
    ///     .line_to(100.0, 100.0)
    ///     .line_to(0.0, 100.0)
    ///     .close()
    /// ```
    #[must_use]
    pub fn close(self) -> Self {
        CGMutablePath::close_subpath(Some(&self.path));
        self
    }

    // ========================================================================
    // Arc operations
    // ========================================================================

    /// Adds an arc of a circle to the path.
    ///
    /// The arc is defined by a center point, radius, and start/end angles.
    /// Angles are in radians, measured clockwise from the positive x-axis.
    ///
    /// # Arguments
    ///
    /// * `center_x` - The x-coordinate of the arc's center
    /// * `center_y` - The y-coordinate of the arc's center
    /// * `radius` - The radius of the arc
    /// * `start_angle` - The starting angle in radians
    /// * `end_angle` - The ending angle in radians
    /// * `clockwise` - If true, draws clockwise; if false, counter-clockwise
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::f64::consts::PI;
    ///
    /// // Draw a semicircle
    /// builder
    ///     .move_to(100.0, 50.0)
    ///     .arc(50.0, 50.0, 50.0, 0.0, PI, false)
    /// ```
    #[must_use]
    pub fn arc(
        self,
        center_x: f64,
        center_y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        clockwise: bool,
    ) -> Self {
        unsafe {
            CGMutablePath::add_arc(
                Some(&self.path),
                self.transform_ptr(),
                center_x,
                center_y,
                radius,
                start_angle,
                end_angle,
                clockwise,
            );
        }
        self
    }

    /// Adds an arc defined by a starting angle and an angular delta.
    ///
    /// This is useful when you know how much angle to sweep rather than
    /// the absolute end angle.
    ///
    /// # Arguments
    ///
    /// * `center_x` - The x-coordinate of the arc's center
    /// * `center_y` - The y-coordinate of the arc's center
    /// * `radius` - The radius of the arc
    /// * `start_angle` - The starting angle in radians
    /// * `delta` - The angle to sweep (positive = counter-clockwise)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::f64::consts::PI;
    ///
    /// // Draw a quarter circle
    /// builder.relative_arc(50.0, 50.0, 50.0, 0.0, PI / 2.0)
    /// ```
    #[must_use]
    pub fn relative_arc(
        self,
        center_x: f64,
        center_y: f64,
        radius: f64,
        start_angle: f64,
        delta: f64,
    ) -> Self {
        unsafe {
            CGMutablePath::add_relative_arc(
                Some(&self.path),
                self.transform_ptr(),
                center_x,
                center_y,
                radius,
                start_angle,
                delta,
            );
        }
        self
    }

    /// Adds an arc that connects to two tangent lines.
    ///
    /// Draws an arc from the current point that is tangent to the line
    /// from the current point to (x1, y1) and tangent to the line from
    /// (x1, y1) to (x2, y2).
    ///
    /// # Arguments
    ///
    /// * `x1` - The x-coordinate of the first tangent point
    /// * `y1` - The y-coordinate of the first tangent point
    /// * `x2` - The x-coordinate of the second tangent point
    /// * `y2` - The y-coordinate of the second tangent point
    /// * `radius` - The radius of the arc
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create a rounded corner
    /// builder
    ///     .move_to(0.0, 50.0)
    ///     .arc_to(0.0, 0.0, 50.0, 0.0, 10.0)  // Rounded corner with radius 10
    ///     .line_to(100.0, 0.0)
    /// ```
    #[must_use]
    pub fn arc_to(self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Self {
        unsafe {
            CGMutablePath::add_arc_to_point(
                Some(&self.path),
                self.transform_ptr(),
                x1,
                y1,
                x2,
                y2,
                radius,
            );
        }
        self
    }

    // ========================================================================
    // Curve operations
    // ========================================================================

    /// Adds a quadratic Bézier curve from the current point.
    ///
    /// A quadratic curve has one control point that determines its shape.
    ///
    /// # Arguments
    ///
    /// * `control_x` - The x-coordinate of the control point
    /// * `control_y` - The y-coordinate of the control point
    /// * `x` - The x-coordinate of the end point
    /// * `y` - The y-coordinate of the end point
    ///
    /// # Examples
    ///
    /// ```ignore
    /// builder
    ///     .move_to(0.0, 0.0)
    ///     .quad_curve_to(50.0, -50.0, 100.0, 0.0)  // Curves up then down
    /// ```
    #[must_use]
    pub fn quad_curve_to(self, control_x: f64, control_y: f64, x: f64, y: f64) -> Self {
        unsafe {
            CGMutablePath::add_quad_curve_to_point(
                Some(&self.path),
                self.transform_ptr(),
                control_x,
                control_y,
                x,
                y,
            );
        }
        self
    }

    /// Adds a cubic Bézier curve from the current point.
    ///
    /// A cubic curve has two control points that determine its shape,
    /// allowing for more complex curves than quadratic.
    ///
    /// # Arguments
    ///
    /// * `cp1_x` - The x-coordinate of the first control point
    /// * `cp1_y` - The y-coordinate of the first control point
    /// * `cp2_x` - The x-coordinate of the second control point
    /// * `cp2_y` - The y-coordinate of the second control point
    /// * `x` - The x-coordinate of the end point
    /// * `y` - The y-coordinate of the end point
    ///
    /// # Examples
    ///
    /// ```ignore
    /// builder
    ///     .move_to(0.0, 0.0)
    ///     .curve_to(25.0, -50.0, 75.0, -50.0, 100.0, 0.0)  // S-curve
    /// ```
    #[must_use]
    pub fn curve_to(self, cp1_x: f64, cp1_y: f64, cp2_x: f64, cp2_y: f64, x: f64, y: f64) -> Self {
        unsafe {
            CGMutablePath::add_curve_to_point(
                Some(&self.path),
                self.transform_ptr(),
                cp1_x,
                cp1_y,
                cp2_x,
                cp2_y,
                x,
                y,
            );
        }
        self
    }

    // ========================================================================
    // Shape operations
    // ========================================================================

    /// Adds a rectangle to the path.
    ///
    /// The rectangle is added as a closed subpath.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the rectangle's origin
    /// * `y` - The y-coordinate of the rectangle's origin
    /// * `width` - The width of the rectangle
    /// * `height` - The height of the rectangle
    ///
    /// # Examples
    ///
    /// ```ignore
    /// builder.rect(10.0, 10.0, 80.0, 60.0)
    /// ```
    #[must_use]
    pub fn rect(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        let rect = CGRect::new(CGPoint::new(x, y), CGSize::new(width, height));
        unsafe {
            CGMutablePath::add_rect(Some(&self.path), self.transform_ptr(), rect);
        }
        self
    }

    /// Adds a rounded rectangle to the path.
    ///
    /// The rectangle is added as a closed subpath with rounded corners.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the rectangle's origin
    /// * `y` - The y-coordinate of the rectangle's origin
    /// * `width` - The width of the rectangle
    /// * `height` - The height of the rectangle
    /// * `corner_radius` - The radius for all corners
    ///
    /// # Examples
    ///
    /// ```ignore
    /// builder.rounded_rect(10.0, 10.0, 80.0, 60.0, 8.0)
    /// ```
    #[must_use]
    pub fn rounded_rect(self, x: f64, y: f64, width: f64, height: f64, corner_radius: f64) -> Self {
        let rect = CGRect::new(CGPoint::new(x, y), CGSize::new(width, height));
        unsafe {
            CGMutablePath::add_rounded_rect(
                Some(&self.path),
                self.transform_ptr(),
                rect,
                corner_radius,
                corner_radius,
            );
        }
        self
    }

    /// Adds a rounded rectangle with different horizontal and vertical corner radii.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the rectangle's origin
    /// * `y` - The y-coordinate of the rectangle's origin
    /// * `width` - The width of the rectangle
    /// * `height` - The height of the rectangle
    /// * `corner_width` - The horizontal corner radius
    /// * `corner_height` - The vertical corner radius
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Elliptical corners
    /// builder.rounded_rect_asymmetric(10.0, 10.0, 80.0, 60.0, 15.0, 8.0)
    /// ```
    #[must_use]
    pub fn rounded_rect_asymmetric(
        self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        corner_width: f64,
        corner_height: f64,
    ) -> Self {
        let rect = CGRect::new(CGPoint::new(x, y), CGSize::new(width, height));
        unsafe {
            CGMutablePath::add_rounded_rect(
                Some(&self.path),
                self.transform_ptr(),
                rect,
                corner_width,
                corner_height,
            );
        }
        self
    }

    /// Adds an ellipse to the path.
    ///
    /// The ellipse is inscribed in the specified bounding rectangle.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the bounding rectangle's origin
    /// * `y` - The y-coordinate of the bounding rectangle's origin
    /// * `width` - The width of the bounding rectangle
    /// * `height` - The height of the bounding rectangle
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Horizontal ellipse
    /// builder.ellipse(10.0, 20.0, 80.0, 40.0)
    /// ```
    #[must_use]
    pub fn ellipse(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        let rect = CGRect::new(CGPoint::new(x, y), CGSize::new(width, height));
        unsafe {
            CGMutablePath::add_ellipse_in_rect(Some(&self.path), self.transform_ptr(), rect);
        }
        self
    }

    /// Adds a circle to the path.
    ///
    /// Convenience method that creates a circular path centered at the
    /// specified point.
    ///
    /// # Arguments
    ///
    /// * `center_x` - The x-coordinate of the center
    /// * `center_y` - The y-coordinate of the center
    /// * `diameter` - The diameter of the circle
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Circle with 50pt diameter centered at (50, 50)
    /// builder.circle(50.0, 50.0, 50.0)
    /// ```
    #[must_use]
    pub fn circle(self, center_x: f64, center_y: f64, diameter: f64) -> Self {
        let radius = diameter / 2.0;
        self.ellipse(center_x - radius, center_y - radius, diameter, diameter)
    }

    // ========================================================================
    // Path composition
    // ========================================================================

    /// Adds another path to this path.
    ///
    /// The contents of the other path are appended to this path.
    ///
    /// # Arguments
    ///
    /// * `other` - The path to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let inner_circle = CGPathBuilder::new()
    ///     .circle(50.0, 50.0, 20.0)
    ///     .build();
    ///
    /// let donut = CGPathBuilder::new()
    ///     .circle(50.0, 50.0, 60.0)
    ///     .add_path(&inner_circle)
    ///     .build();
    /// ```
    #[must_use]
    pub fn add_path(self, other: &CGPath) -> Self {
        unsafe {
            CGMutablePath::add_path(Some(&self.path), self.transform_ptr(), Some(other));
        }
        self
    }

    /// Adds multiple line segments from an array of points.
    ///
    /// Moves to the first point, then draws lines to each subsequent point.
    /// This creates a polyline (connected line segments) through all points.
    ///
    /// # Arguments
    ///
    /// * `points` - Slice of (x, y) coordinate tuples (minimum 2 points)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Creates a zigzag from (0,0) -> (10,10) -> (20,0) -> (30,10)
    /// builder.lines(&[(0.0, 0.0), (10.0, 10.0), (20.0, 0.0), (30.0, 10.0)])
    /// ```
    #[must_use]
    pub fn lines(self, points: &[(f64, f64)]) -> Self {
        if points.is_empty() {
            return self;
        }

        let cg_points: Vec<CGPoint> = points.iter().map(|(x, y)| CGPoint::new(*x, *y)).collect();

        unsafe {
            CGMutablePath::add_lines(
                Some(&self.path),
                self.transform_ptr(),
                cg_points.as_ptr(),
                cg_points.len(),
            );
        }
        self
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Builds and returns the immutable `CGPath`.
    ///
    /// Consumes the builder and returns a `CFRetained<CGPath>` that can
    /// be used with `CAShapeLayer` or other Core Graphics APIs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let path = CGPathBuilder::new()
    ///     .move_to(0.0, 0.0)
    ///     .line_to(100.0, 100.0)
    ///     .build();
    ///
    /// let layer = CAShapeLayerBuilder::new()
    ///     .path(path)
    ///     .build();
    /// ```
    #[must_use]
    pub fn build(self) -> CFRetained<CGPath> {
        // CGMutablePath can be used as CGPath (it's a subtype)
        // We create an immutable copy for safety
        CGPath::new_copy(Some(&self.path)).expect("Failed to create path copy")
    }

    /// Returns the current bounding box of the path.
    ///
    /// Useful for calculating the size needed for a layer's bounds.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = CGPathBuilder::new()
    ///     .circle(50.0, 50.0, 50.0);
    ///
    /// let bounds = builder.bounding_box();
    /// // bounds will be approximately (25, 25, 50, 50)
    /// ```
    #[must_use]
    pub fn bounding_box(&self) -> CGRect {
        CGPath::bounding_box(Some(&self.path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = CGPathBuilder::new();
        assert!(CGPath::is_empty(Some(&builder.path)));
    }

    #[test]
    fn test_move_and_line() {
        let path = CGPathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(100.0, 100.0)
            .build();

        assert!(!CGPath::is_empty(Some(&path)));
    }

    #[test]
    fn test_close_creates_closed_path() {
        let path = CGPathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(100.0, 0.0)
            .line_to(50.0, 100.0)
            .close()
            .build();

        assert!(!CGPath::is_empty(Some(&path)));
    }

    #[test]
    fn test_circle() {
        let path = CGPathBuilder::new().circle(50.0, 50.0, 50.0).build();

        let bounds = CGPath::bounding_box(Some(&path));
        // Circle at (50, 50) with diameter 50 should have bounds (25, 25, 50, 50)
        assert!((bounds.origin.x - 25.0).abs() < 0.001);
        assert!((bounds.origin.y - 25.0).abs() < 0.001);
        assert!((bounds.size.width - 50.0).abs() < 0.001);
        assert!((bounds.size.height - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_rect() {
        let path = CGPathBuilder::new().rect(10.0, 20.0, 100.0, 50.0).build();

        let bounds = CGPath::bounding_box(Some(&path));
        assert!((bounds.origin.x - 10.0).abs() < 0.001);
        assert!((bounds.origin.y - 20.0).abs() < 0.001);
        assert!((bounds.size.width - 100.0).abs() < 0.001);
        assert!((bounds.size.height - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_bounding_box_during_construction() {
        let builder = CGPathBuilder::new()
            .move_to(10.0, 10.0)
            .line_to(90.0, 10.0)
            .line_to(90.0, 90.0)
            .line_to(10.0, 90.0)
            .close();

        let bounds = builder.bounding_box();
        assert!((bounds.origin.x - 10.0).abs() < 0.001);
        assert!((bounds.origin.y - 10.0).abs() < 0.001);
        assert!((bounds.size.width - 80.0).abs() < 0.001);
        assert!((bounds.size.height - 80.0).abs() < 0.001);
    }

    #[test]
    fn test_default_trait() {
        let builder = CGPathBuilder::default();
        assert!(CGPath::is_empty(Some(&builder.path)));
    }

    #[test]
    fn test_lines_helper() {
        // Note: CGPathAddLines moves to first point, then draws lines to subsequent points
        // So this creates a path from (10,10) -> (20,0) -> (30,10)
        let path = CGPathBuilder::new()
            .lines(&[(10.0, 10.0), (20.0, 0.0), (30.0, 10.0)])
            .build();

        assert!(!CGPath::is_empty(Some(&path)));
        let bounds = CGPath::bounding_box(Some(&path));
        // Bounds: x from 10 to 30 (width=20), y from 0 to 10 (height=10)
        assert!((bounds.origin.x - 10.0).abs() < 0.001);
        assert!((bounds.origin.y - 0.0).abs() < 0.001);
        assert!((bounds.size.width - 20.0).abs() < 0.001);
        assert!((bounds.size.height - 10.0).abs() < 0.001);
    }
}
