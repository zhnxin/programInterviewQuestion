#[derive(Debug)]
pub struct Matrix {
    matrix: Vec<Vec<usize>>
}

fn get_min(list:Vec<usize>) -> usize{
    let mut min_num = list[0];
    for num in list.iter(){
        if num < &min_num{
            min_num = *num;
        }
    }
    return min_num;
}

impl Matrix {
    pub fn new(size:usize) -> Matrix{
        let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(size);
        for _ in 0..size{
            let mut temp: Vec<usize> = Vec::with_capacity(size);
            for _ in 0..size {
                temp.push(0);
            }
            matrix.push(temp);
        }
        let mut snake_matrix = Matrix{
            matrix,
        };
        (&mut snake_matrix).generate(size);
        return snake_matrix;
    }

    pub fn size(&self) -> usize{
        self.matrix.len()
    }

    fn get_loop(&self,point: &(usize,usize)) ->usize{
        get_min(vec![point.0,point.1,self.size()-point.0-1,self.size()-point.1-1])
    }

    pub fn get(&self,i:usize,j:usize) -> Result<usize,&str>{
        let size = self.matrix.len();
        if i >= size || j >= size{
            Err("the index is out of range")
        }else{
            Ok(self.matrix[i][j])
        }
    }

    fn next(&mut self,point:&mut (usize,usize),num:usize){
        let &mut(i,j) = point;
        self.matrix[i][j] = num;
        let loop_count = self.get_loop(&point);
        if j>=i && i < self.size()-loop_count-1{
            if j < self.size()-loop_count-1{
                point.1 = j+1;
            }else{
                point.0 = i+1;
            }
        }else {
            if j > loop_count{
                point.1=j-1;
            }else if i > loop_count+1{
                point.0=i-1;
            }else{
                point.1=j+1;
            }
        }

    }

    fn generate(&mut self,size:usize) {
        let last_num = size * size;
        let mut point = (0,0);
        for num in 1..last_num+1 {
            self.next(&mut point,num);
        }
    }

    fn get_list(&self,index:usize)-> &Vec<usize>{
        &self.matrix[index]
    }

    pub fn print_matrix(&self){
        for i in 0..self.size() {
            println!("{:?}",self.get_list(i));
        }
    }

}

