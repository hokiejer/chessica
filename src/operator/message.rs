#[derive(PartialEq,Eq,Copy,Clone,Hash,Debug)]
pub enum OperatorInstruction {
    PlaceHolder,
    NewBoard,
    MoveTaken,
    PlayerStatusChange,
    ExitProgram,
}

use crate::operator::message::OperatorInstruction::PlaceHolder;
use crate::operator::message::OperatorInstruction::PlayerStatusChange;
use crate::operator::message::OperatorInstruction::ExitProgram;

#[derive(PartialEq,Eq,Clone,Hash,Debug)]
pub struct OperatorMessage {
    pub instruction: OperatorInstruction,
    pub data_string: String,
    pub computer_white: bool,
    pub computer_black: bool,
}

pub fn new() -> OperatorMessage {
    OperatorMessage {
        instruction: PlaceHolder,
        data_string: "".to_string(),
        computer_white: false,
        computer_black: false,
    }
}

impl OperatorMessage {
    pub fn new_board(&mut self) {
        self.instruction = ExitProgram;
    }

    pub fn move_taken(&mut self) {
        self.instruction = ExitProgram;
    }

    pub fn player_status_change(&mut self, computer_white: bool, computer_black: bool) {
        self.instruction = PlayerStatusChange;
        self.computer_white = computer_white;
        self.computer_black = computer_black;
    }

    pub fn exit_program(&mut self) {
        self.instruction = ExitProgram;
    }

    pub fn sendable(&mut self) -> bool {
        self.instruction != PlaceHolder
    }
}

