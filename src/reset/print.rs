use crate::reset::Reset;

/// Prints a Reset
/// 
/// # Examples
///
/// ```
/// # use chessica::reset::Reset;
/// let mut r = chessica::reset::new();
/// r.print();
/// ```
impl Reset {
    pub fn print(&mut self) -> String {
        println!("{}",self.to_fen());
        self.to_fen()
    }
}


