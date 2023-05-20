const RAM_SIZE: usize = 12 * 16;

pub struct Ram{
    data: [u16; RAM_SIZE]
}

impl Ram{
    pub fn new() -> Self{
        Self{
            data: [0u16; RAM_SIZE]
        }
    }

    pub fn set(&mut self, value: u16, index: u16){
        if index as usize >= RAM_SIZE{
            return;
        }
        self.data[index as usize] = value;
    }

    pub fn get(&self, index: u16) -> u16{
        if let Some(value) = self.data.get(index as usize){
            return *value;
        }
        0u16
    }
}
