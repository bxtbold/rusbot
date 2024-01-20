mod dh_param;
mod dynamics;
mod utils;

pub use dh_param::DHParameter;
pub use dynamics::Dynamics;
pub use utils::round_matrix4x4;


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

    pub fn get_tf_matrix(&self) -> nalgebra::Matrix4<f64> {

        let m11 = self.dh.theta.cos();
        let m12 = -self.dh.theta.sin() * self.dh.alpha.cos();
        let m13 = self.dh.theta.sin() * self.dh.alpha.sin();
        let m14 = self.dh.r * self.dh.theta.cos();
        let m21 = self.dh.theta.sin();
        let m22 = self.dh.theta.cos() * self.dh.alpha.cos();
        let m23 = -self.dh.theta.cos() * self.dh.alpha.sin();
        let m24 = self.dh.r * self.dh.theta.sin();
        let m31 = 0.0;
        let m32 = self.dh.alpha.sin();
        let m33 = self.dh.alpha.cos();
        let m34 = self.dh.d;
        let m41 = 0.0;
        let m42 = 0.0;
        let m43 = 0.0;
        let m44 = 1.0;

        let mut matrix = nalgebra::Matrix4::new(
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        );
        round_matrix4x4(&mut matrix, 5);

        matrix
    }
}
