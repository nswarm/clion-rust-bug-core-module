use std::error::Error;

// BUG: Not recognized: Eq, Default, Debug, Ord.
#[derive(Eq, PartialEq, Clone, Default, Debug, Copy, Ord, PartialOrd)]
struct Example {

}

fn main() {
    let result : Result<&str, Box<dyn Error>>;
    // BUG: Fails to deduce inner_str type.
    let inner_str = result?;
}
