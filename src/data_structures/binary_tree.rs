// Warning! This implementation is incomplete!

// Does not support duplicate values


pub struct Tree {
    root: Option<Node>
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn insert(&self, value: i32) {
        let node = Node::new(value);

        fn traverse(current_node: Node) {
            if current_node.value > value {
                if current_node.left.is_none() {
                    current_node.left = node;
                } else {
                    traverse(current_node.left);
                }
            } else if current_node.value < value {
                if current_node.right.is_none() {
                    current_node.right = node;
                } else {
                    traverse(current_node.right);
                }
            }
        }

        if self.root.is_none() {
            self.root = node;
        } else {
            traverse(self.root);
        }
    }
}


struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>

}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value: value,
            left: None,
            right: None
        }
    }
}
