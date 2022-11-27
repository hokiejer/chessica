pub mod r#const;
pub mod moves;
pub mod ab_in_place;
pub mod ab_keep_depth;

use crate::reset::Reset;
use std::cell::RefCell;
//use std::sync::Arc;
use std::rc::{Weak};


pub struct Tree
{
    pub reset: Reset,
    pub parent: Option<Weak<Tree>>,
    pub children: Vec<Tree>,
}

pub fn new() -> Tree {

    use crate::tree::r#const::MAX_EXPECTED_CHILD_MOVES;
    let emptyvec = Vec::with_capacity(MAX_EXPECTED_CHILD_MOVES);
    Tree {
        reset: crate::reset::new(),
        parent: None,
        children: emptyvec,
    }
}

pub fn from_fen(fen: String) -> Tree {
    let mut tree = crate::tree::new();
    tree.reset.init_from_fen(fen);
    tree
}

impl Tree {

    pub fn to_fen(&mut self) -> String {
        self.reset.to_fen()
    }

    pub fn print(&mut self) {
        self.reset.print();
    }

    pub fn add_child_last(&mut self, child: Tree) {
        self.children.push(child);
    }

    pub fn purge_children(&mut self) {
        self.children.clear();
    }

    //pub fn add_child_first(&mut self, child: Tree) {

        //let child = crate::tree::new();
        //self.children.insert(0,child);
    //}

    //pub fn get_child_last(&mut self, child: Tree) {

        //let child = crate::tree::new();
        //self.children.push(child);
    //}

    //pub fn swap_with_first(&mut self) {
    //}
}

#[cfg(test)]
mod tests {
    use crate::reset::Reset;
    use crate::tree::Tree;

    #[test]
    fn from_fen_to_fen() {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        assert_eq!(t.to_fen(),"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let fen = String::from("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 23");
        let mut t: Tree = crate::tree::from_fen(fen);
        assert_eq!(t.to_fen(),"r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 23");
    }

}

