pub mod snakematrix;
pub mod shirftstring;
pub mod horsepiece;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_for_matrix(){
        let matrix = snakematrix::Matrix::new(7);
        matrix.print_matrix();
    }

    #[test]
    #[ignore]
    fn test_for_shirft(){
        let mut s = shirftstring::ShirftString::new("hello world".to_string());
        s.right_shirft(5);
        println!("{}",s.get_string());
    }
    // add code here
    #[test]
    fn for_horse() {
        let mut board = super::horsepiece::CheckerBoard::new();

        board.calculate((2,3));
        println!("{}",board.get_count());
        assert_eq!(board.get(&(2,3)),0);
    }

    #[test]
    fn for_horse_query() {
        let mut board = super::horsepiece::CheckerBoardQ::new();
        board.calculate((2,3));
        println!("{}",board.get_count());
        assert_eq!(board.get(&(2,3)),0);
    }
}