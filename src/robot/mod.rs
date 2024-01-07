mod joint;

pub use joint::{Joint, DHParameter, Dynamics};


#[derive(Debug)]
pub struct Robot{
    pub name: String,
    pub joints: Vec<Joint>,
}

impl Robot {
    pub fn new(name: &str) -> Robot {
        println!("Creating a new robot: {}!", name);
        Robot {
            name: name.to_string(),
            joints: Vec::new(),
        }
    }

    pub fn add_joint(&mut self, joint: Joint) {
        self.joints.push(joint);
    }

    pub fn get_dh_parameters(&self) -> Vec<DHParameter> {
        let mut dh_parameters: Vec<DHParameter> = Vec::new();
        for joint in &self.joints {
            dh_parameters.push(joint.get_dh().clone());
        }
        dh_parameters
    }

    pub fn get_dynamics(&self) -> Vec<Dynamics> {
        let mut dynamics: Vec<Dynamics> = Vec::new();
        for joint in &self.joints {
            dynamics.push(joint.get_dynamics().clone());
        }
        dynamics
    }

    pub fn get_joint_names(&self) -> Vec<String> {
        let mut joint_names: Vec<String> = Vec::new();
        for joint in &self.joints {
            joint_names.push(joint.name.clone());
        }
        joint_names
    }
}
