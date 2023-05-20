const ROM_SIZE: usize = 12 * 16;

pub struct Rom {
   data: [u16; ROM_SIZE] 
}

impl Rom{
    pub fn new() -> Self{
        Self{
            data: [0u16; ROM_SIZE]
        }
    }

    pub fn get(&self, index: u16) -> u16{
        if let Some(value) = self.data.get(index as usize){
            return *value;
        }
        0u16
    }

    //For debuging/testing purpurses
    pub fn _set(&mut self, value: u16, index: u16){
        if index as usize >= ROM_SIZE{
            return;
        }
        self.data[index as usize] = value;
    }

    pub fn load_from_file(&mut self, path: &str){
        let bytes: Vec<u8> = std::fs::read(path).unwrap();
        self.load(bytes.chunks_exact(2).into_iter().map(|a| u16::from_ne_bytes([a[0], a[1]])).collect())
    }

    pub fn load(&mut self, words: Vec<u16>){
        for index in 0..ROM_SIZE{
            self.data[index] = **words.get(index).get_or_insert(&0u16);
        }
    }
}
