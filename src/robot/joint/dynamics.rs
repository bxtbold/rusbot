#[derive(Debug, Clone, PartialEq)]
pub struct Dynamics {
    pub mass: f64,
    pub inertia: f64,
    pub coriolis: f64,
    pub gravity: f64,
}

impl Dynamics {
    pub fn new(mass: f64, inertia: f64, coriolis: f64, gravity: f64) -> Option<Dynamics> {
        Some(
            Dynamics {mass, inertia, coriolis, gravity}
        )
    }
}
