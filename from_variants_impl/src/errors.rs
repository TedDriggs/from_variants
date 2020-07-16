use error_chain::*;

error_chain! {
    foreign_links {
        Darling(darling::Error);
    }
}
