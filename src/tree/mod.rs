pub mod r#const;

use crate::reset::Reset;

pub struct Tree<Reset> 
{
    pub reset: Reset,
    pub children: Vec<Tree<Reset>>,
}

pub fn new(reset: Reset) -> Tree<Reset> {

    use crate::tree::r#const::MAX_EXPECTED_CHILD_MOVES;
    let emptyvec = Vec::<Tree<Reset>>::with_capacity(MAX_EXPECTED_CHILD_MOVES);
    Tree {
        reset: reset,
        children: emptyvec,
    }
}

impl Tree<Reset> {

    pub fn add_child_first(mut self, child: Tree<Reset>) {

        //let child = tree::new(reset);
        self.children.insert(0,child);
    }

    pub fn add_child_last(mut self, child: Tree<Reset>) {

        //let child = tree::new(reset);
        self.children.push(child);
    }

    pub fn get_child_last(mut self, child: Tree<Reset>) {

        //let child = tree::new(reset);
        self.children.push(child);
    }

    pub fn swap_with_first(mut self) {
    }
}
