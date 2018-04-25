
fn do_exchange(list: &mut Vec<usize>,index:usize) {
    let temp = list[index+1];
    list[index+1] = list[index];
    list[index] = temp;
}

fn sort(list: &mut Vec<usize>,index:usize,is_stable: bool ) {
    let length = list.len();
    let flag = list[index] < list[index+1];//is need changed
    if !flag{
        do_exchange(list,index);
    }
    let flag = flag && is_stable;
    if index + 2 < length{
        sort(list,index+1,flag);
    }else if flag {
        return;
    }else {
        sort(list,0,true)
    }
}

fn main() {
    let mut list : Vec<usize> = vec![2,3,1,4,6,8,0,5];
    sort(&mut list,0,true);
    println!("{:?}",list);
    let mut a,b :usize;
}