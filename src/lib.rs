mod user {
    mod new;
    mod register;
    mod authorize;
    mod login;
    mod reset_password {
        mod initialize;
        mod reset;
    }
    pub(crate) mod model;
}

mod utils {
    mod mail {
        mod register;
        mod password_reset;
    }
    mod validate {
        mod password;
        mod username;
    }
}

mod buckets {
    mod bucket;
}