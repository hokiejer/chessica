#[derive(PartialEq,Eq,Copy,Clone,Hash,Debug)]
pub enum OperatorInstruction {
    TestInstruction,
    NewBoard,
    MoveTaken,
    PlayerStatusChange,
    ExitProgram,
}

#[derive(PartialEq,Eq,Clone,Hash,Debug)]
pub struct OperatorMessage {
    pub instruction: OperatorInstruction,
    pub data_string: String,
}

pub fn new(instruction: OperatorInstruction, datastring: String) -> OperatorMessage {
    OperatorMessage {
        instruction: instruction,
        data_string: datastring,
    }
}