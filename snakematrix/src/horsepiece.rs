
#[derive(Debug)]
pub struct CheckerBoard {
    board: Vec<Vec<isize>>,
    count:usize,
}


impl CheckerBoard {
    // add code here
    pub fn new() -> CheckerBoard {
        let mut board:Vec<Vec<isize>> = Vec::with_capacity(10);
        let count:usize=0;
        for _ in 0..10 {
            board.push(vec![-1;9]);
        }
        CheckerBoard{
            board,
            count,
        }
    }

    pub fn get(&self,point:&(usize,usize)) -> isize{
        self.board[point.0][point.1]
    }

    pub fn set(&mut self,point:&(usize,usize),length:isize){
        self.board[point.0][point.1] = length;
    }

    pub fn print_board(&self){
        for i in 0..10{
            println!("{:?}",self.board[i]);
        }
    }
    pub fn calculate(&mut self,point:(usize,usize)){
        self.travel(point,-1);
    }
    fn travel(&mut self,point:(usize,usize),length:isize){
        if point.0>9 || point.1>8{
            return;
        }
        self.count = self.count + 1;
        let _length = self.get(&point);
        if _length>=0 && _length<=length+1{
            return;
        }
        self.count = self.count+1;
        let _length = length+1;
        self.set(&point,length+1);
        if point.0>1{
            if point.1>1{
                self.travel((point.0-1,point.1-2),_length);
                self.travel((point.0-1,point.1+2),_length);
                self.travel((point.0-2,point.1-1),_length);
                self.travel((point.0-2,point.1+1),_length);
                self.travel((point.0+1,point.1-2),_length);
                self.travel((point.0+2,point.1-1),_length);
            }else if point.1>0 {                
                self.travel((point.0-1,point.1+2),_length);
                self.travel((point.0-2,point.1-1),_length);
                self.travel((point.0-2,point.1+1),_length);
                self.travel((point.0+2,point.1-1),_length);
            }
        }else if point.0>0{
            if point.1>1{
                self.travel((point.0-1,point.1-2),_length);
                self.travel((point.0-1,point.1+2),_length);
                self.travel((point.0+1,point.1-2),_length);
                self.travel((point.0+2,point.1-1),_length);
            }else if point.1>0{
                self.travel((point.0-1,point.1+2),_length);
                self.travel((point.0+2,point.1-1),_length);
            }
        }
        self.travel((point.0+1,point.1+2),_length);
        self.travel((point.0+2,point.1+1),_length);
        
    }

    pub fn get_count(&self) -> usize{
        self.count
    }
    
}

#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    pub fn new() -> Self {
        Queue{qdata: Vec::new()}
    }

    pub fn len(&self)->usize{
        self.qdata.len()
    }

    pub fn push(&mut self, item:T) {
        self.qdata.push(item);
    }

    pub fn pop(&mut self) -> T{
        self.qdata.remove(0)
    }
}

#[derive(Debug)]
pub struct CheckerBoardQ {
    board: Vec<Vec<isize>>,
    count:usize,
    point_query:Queue<(isize,isize)>,
}
impl CheckerBoardQ {
    // add code here
    pub fn new() -> CheckerBoardQ {
        let mut board:Vec<Vec<isize>> = Vec::with_capacity(10);
        let count:usize=0;
        let point_query:Queue<(isize,isize)> = Queue::new();
        for _ in 0..10 {
            board.push(vec![-1;9]);
        }
        CheckerBoardQ{
            board,
            count,
            point_query,
        }
    }
    fn set(&mut self,point:&(isize,isize),length:isize){
        self.board[point.0 as usize ][point.1 as usize] = length;
    }
    pub fn get(&self,point:&(usize,usize)) -> isize{
        self.board[point.0][point.1]
    }
    fn _get(&self,point:&(isize,isize)) -> isize{
        self.board[point.0 as usize][point.1 as usize]
    }
    pub fn print_board(&self){
        for i in 0..10{
            println!("{:?}",self.board[i]);
        }
    }
    pub fn calculate(&mut self,point:(usize,usize)){
        assert!(point.0<10 && point.1<9);
        let i_point = (point.0 as isize,point.1 as isize);
        self.set(&i_point,0);
        self.point_query.push(i_point);
        while self.point_query.len()>0 {
            self.count = self.count + 1;
            let _point = self.point_query.pop();
            self.travel(_point);
        }

        
    }
    fn update_query(&mut self,length:isize,next_point:(isize,isize)){
        let length = length+1;
        let _length = self._get(&next_point);
        if _length<0 || _length>length {
            self.set(&next_point,length);
            self.point_query.push(next_point);
        }
    }
    fn travel(&mut self,point:(isize,isize))  {
        //todo travel all 8 point
        let mut _next_point_query = vec![
            (point.0-1,point.1-2),(point.0-1,point.1+2),
            (point.0+1,point.1-2),(point.0+1,point.1+2),
            (point.0-2,point.1-1),(point.0-2,point.1+1),
            (point.0+2,point.1-1),(point.0+2,point.1+1)].into_iter();
        let length = self._get(&point);
        while let Some(_point) = _next_point_query.next() {
            if _point.0<0||_point.0>9||_point.1<0||_point.1>8{
                continue;
            }
            self.update_query(length,_point);
        }
    }
    pub fn get_count(&self) -> usize {
        self.count
    }
    
}

