mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            // Access to secret
            outermost::middle_secret_function();
            outermost::inside::secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::inside::inner_function();

    // No access to secret functions
    outermost::middle_secret_function();
    outermost::inside::secret_function();
}