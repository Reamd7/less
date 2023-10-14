pub mod length {
    pub static m: i32 = 1;
    pub static cm: f64 = 0.01;
    pub static mm: f64 = 0.001;
    pub static r#in: f64 = 0.0254;
    pub static px: f64 = 0.0254 / 96.0;
    pub static pt: f64 = 0.0254 / 72.0;
    pub static pc: f64 = 0.0254 / 72.0 * 12.0;
}

pub mod duration {
    pub static s: i32 = 1;
    pub static ms: f64 = 0.001;
}

pub mod angle {
    use std::f64::consts::PI;

    pub static rad: f64 = 1.0 / 2.0 * PI;
    pub static deg: f64 = 1.0 / 360.0;
    pub static grad: f64 = 1.0 / 400.0;
    pub static turn: i32 = 1;
}