pub mod user {
    pub mod new;
    pub mod register;
    pub mod authorize;
    pub mod login;
    pub mod reset_password {
        pub mod initialize;
        pub mod reset;
    }
    pub mod model;
}

pub mod utils {
    pub mod mail {
        pub mod register;
        pub mod password_reset;
    }
    pub mod validate {
        pub mod password;
        pub mod username;
    }
    pub mod appstate;
}

pub mod buckets {
    pub mod bucket;
}