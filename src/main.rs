fn main() {
    println!("Cavity: The suckless Rust project template");
}

#[allow(dead_code)]
fn cavity() -> String {
    "cavity".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_cavity() {
        let word = String::from("cavity");
        let result = cavity();
        assert_eq!(word, result);
    }
}
