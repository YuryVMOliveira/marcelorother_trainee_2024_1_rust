enum Status{
    menu,
    jogando_sem_caixa,
    jogando_com_caixa,
    fim
}

pub struct Carteiro{
    pos_x: i32,
    pos_y: i32,
    status: Status
}

impl Carteiro{
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Carteiro {
            pos_x,
            pos_y,
            status: Status::menu, // Começa no estado de menu
        }
    }
    pub fn mover_para(&mut self, x: i32, y: i32) {
        self.pos_x = x;
        self.pos_y = y;
    }
    pub fn get_pos_x(&self) -> i32 {
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32 {
        self.pos_y
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }
}