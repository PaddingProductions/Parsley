pub trait Operation {
    fn exec (&self);
}

pub struct PrintOperation {
    s: String
}
impl PrintOperation {
    pub fn new (s: &str) -> Self {
        Self { s: String::from(s) }
    }
}
impl Operation for PrintOperation {
    fn exec(&self) {
        println!("{}", self.s);
    }
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    pub t0: f64,
    pub v: Vec<(char, f64)>
}
impl Expression {
    pub fn new (t0: f64, v: Vec<(char, f64)>) -> Self {
        Self { t0, v }
    }
    pub fn eval (&self) -> f64 {
        let mut res = self.t0;
        for (op_c, v) in self.v.iter() {
            match op_c {
                '+' => res += v,
                '*' => res *= v,
                '-' => res -= v,
                '/' => res /= v,
                _ => panic!("invalid operator character found in expression")
            }
        }
        res
    }
}
