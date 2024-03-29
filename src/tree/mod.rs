pub mod r#const;
pub mod moves;
pub mod ab_in_place;
pub mod ab_promote_prune;
pub mod ab_iterative_promote_prune;
pub mod ab_promote_prune_parallel;

use crate::reset::Reset;

pub struct Tree
{
    pub reset: Reset,
    pub children: Vec<Tree>,
    pub score: Option<i32>,
}

unsafe impl Send for Tree {}

pub fn new() -> Tree {

    use crate::tree::r#const::MAX_EXPECTED_CHILD_MOVES;
    let emptyvec = Vec::with_capacity(MAX_EXPECTED_CHILD_MOVES);
    Tree {
        reset: crate::reset::new(),
        children: emptyvec,
        score: None,
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

    // The child I specify shall be first and the first shall be last
    pub fn promote_last_child_to_first(&mut self, last_child_index: usize) {
        let my_slice = &mut self.children[..=last_child_index];
        my_slice.rotate_right(1);
    }

    pub fn purge_children(&mut self) {
        self.children.clear();
    }

    pub fn number_of_children(&mut self) -> u32 {
        self.children.len().try_into().unwrap()
    }

    pub fn number_of_children_usize(&mut self) -> usize {
        self.children.len()
    }

    pub fn count_tree_nodes(&mut self, level: u8, node_count: &mut Vec<u64>) {
        if node_count.len() <= level as usize {
            node_count.push(0);
        }
        node_count[level as usize] += self.children.len() as u64;
        for c in 0..self.children.len() {
            let child = &mut self.children[c];
            child.count_tree_nodes(level+1, node_count);
        }
    }

    pub fn print_diagnostics(&mut self) {
        let mut node_count: Vec<u64> = Vec::new();
        node_count.push(1);
        self.count_tree_nodes(1, &mut node_count);
        (0..node_count.len()).for_each(|i| {
            println!("Depth = {}, node_count = {}",i,node_count[i]);
        });
    }

    pub fn print_children(&mut self) -> bool {
        if !self.children.is_empty() {
            self.reset.print_board_big();
            for c in 0..self.children.len() {
                let child = &mut self.children[c];
                child.reset.print();
            }
            return true
        }
        false
    }

    pub fn best_line(&mut self) -> String {
        if self.children.is_empty() {
            self.reset.move_text()
        } else {
            let my_best_line = self.children[0].best_line();
            format!("{} {}",self.reset.move_text(),my_best_line.trim())
        }

    }

    //pub fn add_child_first(&mut self, child: Tree) {

        //let child = crate::tree::new();
        //self.children.insert(0,child);
    //}

    //pub fn get_child_last(&mut self, child: Tree) {

        //let child = crate::tree::new();
        //self.children.push(child);
    //}
}

#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    #[test]
    fn from_fen_to_fen() {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        assert_eq!(t.to_fen(),"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(t.score,None);

        let fen = String::from("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 23");
        let mut t: Tree = crate::tree::from_fen(fen);
        assert_eq!(t.to_fen(),"r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 23");
        assert_eq!(t.score,None);
    }

    #[test]
    fn best_line() {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        t.add_next_child();
        t.add_next_child();
        let c = t.children.pop().unwrap();
        t.children[0].children.push(c);
        t.add_next_child();
        let c = t.children.pop().unwrap();
        t.children[0].children[0].children.push(c);
        t.add_next_child();
        let c = t.children.pop().unwrap();
        t.children[0].children[0].children[0].children.push(c);
        let s = t.children[0].best_line();
        assert_eq!(s,"g1h3 g1f3 b1c3 b1a3".to_string());
    }

}

