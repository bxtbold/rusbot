mod dh_param;
mod dynamics;

pub use dh_param::DHParameter;
pub use dynamics::Dynamics;


#[derive(Debug)]
pub struct Joint {
    pub name: String,
    pub dh: DHParameter,
    pub dynamics: Dynamics,
}

impl Joint {
    pub fn new(name: &str, dh: Option<DHParameter>, dynamics: Option<Dynamics>) -> Joint {
        Joint {
            name: name.to_string(),
            dh: dh.unwrap_or(
                DHParameter::new(0.0, 0.0, 0.0, 0.0)
                .expect("DHParameters are not valid!")
            ),
            dynamics: dynamics.unwrap_or(
                Dynamics::new(0.0, 0.0, 0.0, 0.0)
                .expect("Dynamics are not valid!")
            ),
        }
    }

    pub fn set_dh(&mut self, dh: DHParameter) {
        self.dh = dh;
    }

    pub fn set_dynamics(&mut self, dynamics: Dynamics) {
        self.dynamics = dynamics;
    }

    pub fn get_dh(&self) -> &DHParameter {
        &self.dh
    }

    pub fn get_dynamics(&self) -> &Dynamics {
        &self.dynamics
    }
}
