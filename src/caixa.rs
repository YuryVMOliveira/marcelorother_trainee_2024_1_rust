// caixa.rs

pub struct Caixa {
    pos_x: i32,
    pos_y: i32,
}

impl Caixa {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Caixa { pos_x, pos_y }
    }

    // Métodos getters
    pub fn get_pos_x(&self) -> i32 {
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32 {
        self.pos_y
    }
}
