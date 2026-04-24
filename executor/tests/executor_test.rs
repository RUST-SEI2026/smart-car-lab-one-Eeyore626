use  executor ::{Executor,Pose};
mod turn_left_tests {
    use super::*;

    #[test]
    fn should_return_facing_n_given_command_is_l_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_l_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }



}
