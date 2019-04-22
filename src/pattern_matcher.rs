pub trait IsIn {
    fn is_in(&self, string: &str) -> bool;
}

impl IsIn for str {
    fn is_in(&self, string: &str) -> bool {
        string.contains(self)
    }
}
