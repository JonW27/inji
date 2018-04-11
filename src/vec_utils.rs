// unfinished
pub struct math_vec{
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl math_vec{
    fn magnitude(&self) -> f64 {
        (x.powf(2.)+y.powf(2.)+z.powf(2.)).sqrt();
    }
    fn normalize(&self){
        let scale = 1/self.magnitude();
        self.x = self.x * scale;
        self.y = self.y * scale;
        self.z = self.z * scale;
    }
    fn crossProduct(&self, other : math_vec) -> math_vec{
        let p_of_m = self.magnitude() * other.magnitude(); // product of magnitudes
        math_vec{
            x : p_of_m * self.x,
            y : p_of_m * y,
            z : p_of_m * z,
        }
    }
}