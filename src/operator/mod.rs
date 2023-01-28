
/// Data necessary the Operator functionality to run successfully
///
/// The Operator will serve as the intermediary between the chess board interface and the
/// Orchestrator thread, which oversees move searching.
///
pub struct Operator {
    x: u32,
}

/// Constructs a new Operator
///
/// # Examples
///
/// ```
/// use chessica::operator::Operator;
/// let mut my_operator = chessica::operator::new();
/// ```
pub fn new() -> Operator {
    Operator {
        x: 0,
    }
}

impl Operator {

    /// Launch Chessica's Operator
    ///
    /// This will, in turn, launch the Orchestrator to ensure that the search engine does it's thing
    pub fn launch(&self) {

    }


}

#[cfg(test)]
mod tests {
    use crate::operator;

    #[test]
    fn new_operator() {
        let o = operator::new();
        assert_eq!(o.x,0);
    }

}
