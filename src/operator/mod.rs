pub mod message;
pub mod instructions;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use crate::operator::message::OperatorMessage;

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
    orchestrator_transmit_channel: Option<Sender<OperatorMessage>>,
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
        orchestrator_transmit_channel: None
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
        use std::io;
        use crate::orchestrator;

        let (tx, rx) = mpsc::channel();
        self.orchestrator_transmit_channel = Some(tx);

        // Spawn the Orchestrator thread
        let orchestrator_join_handle = thread::spawn(|| {
            let mut orchestrator = orchestrator::new(rx);
            orchestrator.run();
        });


        println!("Spawned!");
        // Now we need to do all the Operator things

        loop {
            let mut input = "".to_string();
            io::stdin().read_line(&mut input).unwrap();
            let instruction = input.trim().to_string();
            self.process_instruction(&instruction);

            if instruction == "quit" {
                break;
            }
        }

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
        assert!(!o.white_is_engine);
        assert!(!o.black_is_engine);
        assert_eq!(o.game_fen,"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        assert_eq!(o.communication_protocol,ChessEngineCommunicationProtocol);
    }

}
