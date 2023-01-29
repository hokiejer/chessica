use std::sync::mpsc::Receiver;
use crate::operator::message::OperatorMessage;

/// Data necessary the Orchestrator functionality to run successfully
///
/// The Ochestrator will take game status from the Operator and will launch Cogitator threads as
/// appropriate to build out the game tree and identify the best possible moves.  The Orchestrator
/// runs in its own thread.
///
pub struct Orchestrator {
    receiver_channel: Receiver<OperatorMessage>,
}

/// Constructs a new Orchestrator
///
/// # Examples
///
/// ```
/// use chessica::orchestrator::Orchestrator;
/// use std::sync::mpsc;
/// let (_tx, rx) =  mpsc::channel();
/// let mut my_orchestrator = chessica::orchestrator::new(rx);
/// ```
pub fn new(receiver: Receiver<OperatorMessage>) -> Orchestrator {
    Orchestrator {
        receiver_channel: receiver,
    }
}

impl Orchestrator {

    /// Run Chessica's Orchestrator
    ///
    /// This will launch and manage Cogitator threads as appropriate
    pub fn run(&self) {
            println!("I am the orchestrator and I'm running.  WHEEEEEE!");
            let received_value = self.receiver_channel.recv().unwrap();
            println!("received value = {:?}",received_value);
    }


}

#[cfg(test)]
mod tests {
    use crate::orchestrator;

    #[test]
    fn new_orchestrator() {
        use std::sync::mpsc;
        let (_t,r) = mpsc::channel();
        let o = orchestrator::new(r);
        // Can't assert Receiver<>
    }

}
