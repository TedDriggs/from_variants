use darling;

error_chain! {
    foreign_links {
        Darling(darling::Error);
    }
}