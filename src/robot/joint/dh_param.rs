#[derive(Debug, Clone, PartialEq)]
pub struct DHParameter {
    pub d: f64,      // offset along previous z to the common normal
    pub alpha: f64,  // angle about previous z from old x to new x
    pub r: f64,      // length of the common normal
    pub theta: f64,  // angle about common normal from old z axis to new z axis
}

impl DHParameter {
    pub fn new(d: f64, alpha: f64, r: f64, theta: f64) -> Option<DHParameter> {
        Some(
            DHParameter {d, alpha, r, theta}
        )
    }
}
