use std::io;

error_chain! {

    foreign_links {
        Io(io::Error);
    }

    errors {
        NoMoreInput {
            description("")
            display("")
        }

        UserAborted {
            description("")
            display("")
        }

        InvalidChoice(index:usize) {
            description("")
            display("")
        }
    }

}
