
/// Data necessary the Orchestrator functionality to run successfully
///
/// The Ochestrator will take game status from the Operator and will launch Cogitator threads as
/// appropriate to build out the game tree and identify the best possible moves.  The Orchestrator
/// runs in its own thread.
///
pub struct Orchestrator {
    x: u32,
}

/// Constructs a new Orchestrator
///
/// # Examples
///
/// ```
/// use chessica::orchestrator::Orchestrator;

/// let mut my_orchestrator = chessica::orchestrator::new();
/// ```
pub fn new() -> Orchestrator {
    Orchestrator {
        x: 0,
    }
}

impl Orchestrator {

    /// Run Chessica's Orchestrator
    ///
    /// This will launch and manage Cogitator threads as appropriate
    pub fn run(&self) {
            println!("I am the orchestrator and I'm running.  WHEEEEEE!");
    }


}

#[cfg(test)]
mod tests {
    use crate::orchestrator;

    #[test]
    fn new_orchestrator() {
        let o = orchestrator::new();
        assert_eq!(o.x,0);
    }

}
