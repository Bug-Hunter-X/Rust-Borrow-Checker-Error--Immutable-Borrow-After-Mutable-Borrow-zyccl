# Rust Borrow Checker Error Example

This repository demonstrates a common error encountered when working with references in Rust: attempting to use an immutable reference after a mutable reference has been established to the same data.  The Rust compiler's borrow checker prevents this to maintain data consistency and prevent data races. 

The `bug.rs` file contains the erroneous code, while `bugSolution.rs` provides a corrected version.  This example highlights the importance of understanding Rust's borrowing rules.