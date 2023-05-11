pub struct Environment {
    pub vars: std::collections::HashMap<String, f64>
}
impl Environment {
    pub fn new () -> Self {
        Self { vars: std::collections::HashMap::new() }
    }
}
