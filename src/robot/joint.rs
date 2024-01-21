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
    pub tf_matrix: nalgebra::Matrix4<f64>,
}

impl Joint {
    pub fn new(name: &str, dh: Option<DHParameter>, dynamics: Option<Dynamics>) -> Joint {
        let dh = dh.unwrap_or(
            DHParameter::new(0.0, 0.0, 0.0, 0.0)
            .expect("DHParameters are not valid!")
        );
        let dynamics = dynamics.unwrap_or(
            Dynamics::new(0.0, 0.0, 0.0, 0.0)
            .expect("Dynamics are not valid!")
        );
        let tf_matrix = Joint::load_tf_matrix(dh.clone());

        Joint {
            name: name.to_string(),
            dh,
            dynamics,
            tf_matrix,
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

    pub fn get_tf_matrix(&self) -> &nalgebra::Matrix4<f64> {
        &self.tf_matrix
    }

    fn load_tf_matrix(dh: DHParameter) -> nalgebra::Matrix4<f64> {

        let m11 = dh.theta.cos();
        let m12 = -dh.theta.sin() * dh.alpha.cos();
        let m13 = dh.theta.sin() * dh.alpha.sin();
        let m14 = dh.r * dh.theta.cos();
        let m21 = dh.theta.sin();
        let m22 = dh.theta.cos() * dh.alpha.cos();
        let m23 = -dh.theta.cos() * dh.alpha.sin();
        let m24 = dh.r * dh.theta.sin();
        let m31 = 0.0;
        let m32 = dh.alpha.sin();
        let m33 = dh.alpha.cos();
        let m34 = dh.d;
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
