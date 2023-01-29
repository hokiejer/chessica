pub mod message;

use std::thread;
use std::sync::mpsc;

#[derive(PartialEq,Eq,Copy,Clone,Hash,Debug)]
pub enum CommunicationProtocol {
    UCI,
    ChessEngineCommunicationProtocol,
}

/// Data necessary the Operator functionality to run successfully
///
/// The Operator will serve as the intermediary between the chess board interface and the
/// Orchestrator thread, which oversees move searching.
///
pub struct Operator {
    white_is_engine: bool,
    black_is_engine: bool,
    game_fen: String,
    communication_protocol: CommunicationProtocol,
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
    use crate::operator::CommunicationProtocol::ChessEngineCommunicationProtocol;
    Operator {
        white_is_engine: false,
        black_is_engine: false,
        game_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        communication_protocol: ChessEngineCommunicationProtocol,
    }
}

impl Operator {

    /// Run Chessica's Operator
    ///
    /// This will first launch the Orchestrator thread, whose job it is to orchestrate the chess engine
    /// search.  Then it will interact with the chess board via the selected protocol.  Once done
    /// interacting, the program is done running, so it'll wait for the Orchestrator to die and then
    /// exit.
    pub fn run(&mut self) {
        use crate::orchestrator;
        use crate::operator::message::OperatorMessage;
        use crate::operator::message::OperatorInstruction::Instruction1;

        let (tx, rx) = mpsc::channel();

        // Spawn the Orchestrator thread
        let orchestrator_join_handle = thread::spawn(|| {
            let orchestrator = orchestrator::new(rx);
            orchestrator.run();
        });


        println!("Spawned!");
        let message = OperatorMessage {
            instruction: Instruction1
        };
        tx.send(message).unwrap();

        // Now we need to do all the Operator things

        // Wait for the Orchestrator thread to end
        let _res = orchestrator_join_handle.join();
    }


}

#[cfg(test)]
mod tests {
    use crate::operator;
    use crate::operator::CommunicationProtocol::ChessEngineCommunicationProtocol;

    #[test]
    fn new_operator() {
        let o = operator::new();
        assert_eq!(o.white_is_engine,false);
        assert_eq!(o.black_is_engine,false);
        assert_eq!(o.game_fen,"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        assert_eq!(o.communication_protocol,ChessEngineCommunicationProtocol);
    }

}
