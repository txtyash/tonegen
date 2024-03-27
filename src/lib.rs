#[allow(dead_code)]
pub fn cavity() -> String {
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
