use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the three Euler angles in degrees that represent the
/// orientation of the MT
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Quaternions<T: PrecisionExt> {
    pub q0: T,
    pub q1: T,
    pub q2: T,
    pub q3: T,
}

impl<T: PrecisionExt> fmt::Display for Quaternions<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Q0({:.3}), Q1({:.3}), Q2({:.3}), Q2({:.3})",
            self.q0, self.q1, self.q2, self.q3
        )
    }
}

precision_float32_4field_wire_impl!(Quaternions, q0, q1, q2, q3);
precision_float64_4field_wire_impl!(Quaternions, q0, q1, q2, q3);
// precision_float64_4field_wire_impl!(Quaternions, roll, pitch, yaw);
precision_fp1220_4field_wire_impl!(Quaternions, q0, q1, q2, q3);
// precision_fp1632_4field_wire_impl!(Quaternions, roll, pitch, yaw);
