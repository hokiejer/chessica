pub mod actions;
pub mod r#const;

use std::sync::mpsc::Receiver;
use crate::operator::message::OperatorMessage;
use crate::tree;
use tree::Tree;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Data necessary the Orchestrator functionality to run successfully
///
/// The Ochestrator will take game status from the Operator and will launch Cogitator threads as
/// appropriate to build out the game tree and identify the best possible moves.  The Orchestrator
/// runs in its own thread.
///
pub struct Orchestrator {
    operator_receive_channel: Receiver<OperatorMessage>,
    tree_root: Tree,
    tree_children: Vec<Arc<Mutex<Tree>>>,
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
    let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    Orchestrator {
        operator_receive_channel: receiver,
        tree_root: tree::from_fen(starting_fen),
        tree_children: Vec::new(),
    }
}

impl Orchestrator {

    /// Run Chessica's Orchestrator
    ///
    /// This will launch and manage Cogitator threads as appropriate
    pub fn run(&mut self) {
        println!("I am the orchestrator and I'm running.  WHEEEEEE!");
        loop {
            let received_message = self.operator_receive_channel.recv().unwrap();
            println!("received message = {:?}",received_message);
            //returns true if instructed to exit
            if self.process_command(received_message) {
                println!("Oh crap, I need to quit.  See ya!");
                break;
            }
        };
    }


}

#[cfg(test)]
mod tests {
    use crate::orchestrator;

    #[test]
    fn new_orchestrator() {
        use std::sync::mpsc;
        let (_t,r) = mpsc::channel();
        let _o = orchestrator::new(r);
        // Can't assert Receiver<>
    }

}
