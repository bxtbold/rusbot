mod robot;

pub use robot::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joint_name() {
        // construct a robot
        let mut robot = Robot::new("ur5e");
        let joint1 = Joint::new("joint1", None, None);
        let joint2 = Joint::new("joint2", None, None);
        let joint3 = Joint::new("joint3", None, None);
        robot.add_joint(joint1);
        robot.add_joint(joint2);
        robot.add_joint(joint3);

        // test the joint name
        let joint_names = robot.get_joint_names();
        assert!(joint_names[0] == "joint1");
        assert!(joint_names[1] == "joint2");
        assert!(joint_names[2] == "joint3");
    }

    #[test]
    fn test_joint_dh_params() {
        // construct a robot
        let mut robot = Robot::new("ur5e");
        let dh1 = DHParameter::new(5.16617458590322, 0.0, 0.1625, 1.5707963267949);
        let dh2 = DHParameter::new(1.06465084371654, -0.4250, 0.0, 0.0);
        let joint1 = Joint::new("joint1", dh1.clone(), None);
        let joint2 = Joint::new("joint2", dh2.clone(), None);
        robot.add_joint(joint1);
        robot.add_joint(joint2);

        // test the joint dh parameters
        let joint_dh_params = robot.get_dh_parameters();
        assert!(joint_dh_params[0] == dh1.unwrap());
        assert!(joint_dh_params[1] == dh2.unwrap());
    }

    #[test]
    fn test_joint_dynamics() {
        // construct a robot
        let mut robot = Robot::new("ur5e");
        let dynamics1 = Dynamics::new(1.0, 2.0, 3.0, 4.0);
        let dynamics2 = Dynamics::new(5.0, 6.0, 7.0, 8.0);
        let joint1 = Joint::new("joint1", None, dynamics1.clone());
        let joint2 = Joint::new("joint2", None, dynamics2.clone());
        robot.add_joint(joint1);
        robot.add_joint(joint2);

        // test the joint dynamics
        let joint_dynamics = robot.get_dynamics();
        assert!(joint_dynamics[0] == dynamics1.unwrap());
        assert!(joint_dynamics[1] == dynamics2.unwrap());
    }

    #[test]
    fn test_joint_tf_matrices() {
        // construct a robot
        let mut robot = Robot::new("ur5e");
        let dh1 = DHParameter::new(0.1625, 1.5707964268, 0.0, 0.0);
        let joint1 = Joint::new("joint1", dh1.clone(), None);
        robot.add_joint(joint1);

        // test the joint tf matrices
        let joint_tf_matrices = robot.get_tf_matrices();
        let tf1 = joint_tf_matrices[0];


        let mut tf1_expected = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 1.0, 0.0, 0.1625,
            0.0, 0.0, 0.0, 1.0
        );

        round_matrix4x4(&mut tf1_expected, 5);

        assert!(tf1 == tf1_expected);
    }
}
