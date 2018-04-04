fn next(index: usize) -> usize {
    match index {
        0|1 => 1,
        _ => next(index-1)+next(index-2),
    }
}

fn main(){
    use std::env;
    let args: Vec<String> = env::args().collect();
    assert!(args.len()>1);//断言传入参数
    match args[1].parse::<usize>() {
        Ok(num) => println!("output:{}",next(num)),
        Err(..) => println!("this was not an integer: {}",args[1]),
    }   
    
}