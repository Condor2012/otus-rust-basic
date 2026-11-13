pub struct mystruct{
    f2:u32,
}

impl mystruct {
    pub fn new(f2:u32) -> Self{
        mystruct {f2}
    }

    pub fn get_mystruct(&self) -> u32{
        self.f2
    }
    
}