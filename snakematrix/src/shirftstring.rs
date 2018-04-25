use std::string::String;

#[derive(Debug)]
pub struct ShirftString {
    string :String,
    length : usize,
}

impl ShirftString {
    pub fn new(string:String)->ShirftString{
        string.is_ascii();
        let length = string.len();
        ShirftString{
            string,
            length,
        }
    }
    fn swap(&mut self,a:usize,b:usize){
        assert!(a<self.length && b<self.length);
        unsafe{
            let slice = self.string.as_bytes_mut();
            slice.swap(a,b);
        }
    }

    fn get_swap_index(&self,index:usize,shift_amount:usize) -> usize{
        let mut temp = (index+self.length-shift_amount)%self.length;
        while temp < index{
            temp = (temp+self.length-shift_amount)%self.length
        }
        return temp;
    }
    pub fn right_shirft(&mut self,shift_amount:usize){
        let _shift_amount = shift_amount%self.length;
        let mut swap_index:usize;
        for i in 0..self.length{
            swap_index = self.get_swap_index(i,_shift_amount);
            self.swap(i,swap_index);
        }
    }

    pub fn get_string(&self)->&String{
        &self.string
    }
    
}