//! Extension trait for `Duration` literals.
//!
//! Re-exported from the [`duration_ext`] crate.
//!
//! ```ignore
//! 5.seconds()      // 5 sec
//! 500.millis()     // 500 ms
//! 1.5.seconds()    // 1.5 sec (f64)
//! 2.minutes()      // 2 min
//! 1.hours()        // 1 hour
//! ```

pub use duration_ext::DurationExt;
