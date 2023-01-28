/// Data necessary for the Cogitator functionality to run successfully
///
/// The Cogitator will be launched by the Orchestrator and will build out the game tree and
/// identify the best possible move.  The chess engine itself runs within this thread.
/// Cogitators are built to be scaled horizontally.
///
pub struct Cogitator {
    x: u32,
}

/// Constructs a new Cogitator
///
/// # Examples
///
/// ```
/// use chessica::cogitator::Cogitator;

/// let mut my_cogitator = chessica::cogitator::new();
/// ```
pub fn new() -> Cogitator {
    Cogitator {
        x: 0,
    }
}

impl Cogitator {

    /// Launch Chessica's Cogitator
    pub fn launch(&self) {

    }


}

#[cfg(test)]
mod tests {
    use crate::cogitator;

    #[test]
    fn new_cogitator() {
        let o = cogitator::new();
        assert_eq!(o.x,0);
    }

}
