pub struct Node {
    name: String,
    pos_x: f64,
    pos_y: f64,
}

impl Node {
    pub fn create(name: String, pos_x: f64, pos_y: f64) -> Node {
        Node { name, pos_x, pos_y }
    }
    pub fn update_pos(&self, pos_x: f64, pos_y: f64) -> Node {
        Node {
            name: format!("{}", self.name),
            pos_x,
            pos_y,
        }
    }
}

/////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_node_pos() {
        let node = Node::create(String::from("test"), 10.0, 20.0);
        let updated_node = node.update_pos(11.0, 21.0);
        assert_eq!(node.name, String::from("test"));
        assert_eq!(node.name, updated_node.name);
        assert_eq!(node.pos_x, 10f64);
        assert_eq!(node.pos_y, 20f64);
        assert_eq!(updated_node.pos_x, 11f64);
        assert_eq!(updated_node.pos_y, 21f64);
    }
}
