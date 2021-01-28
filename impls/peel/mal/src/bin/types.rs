#[derive(Copy, Clone)]
pub enum TTYPES {
    uninit,
    list,
    symbol,
    integer,
    string,
    comment,
    twoSpecialChar,
    specialChar,
    identifier,
    keyword,
    end,
    vector,
    hashmap,
    // nil, true, false
}

pub struct Node {
    token: String,
    token_type: TTYPES,
    children: Vec<Node>,
}


impl Clone for Node {
    fn clone (& self) -> Self {
        let mut n = Node::new(self.token.clone(), self.token_type.clone());
        n.children = self.children.clone();
        n
    }
}


impl Node {
    pub fn get_token(&self) -> &str {
        return &(self.token)
    }
    pub fn get_token_type(&self) -> &TTYPES {
        return &self.token_type
    }
    pub fn concat(&mut self, s: &str) {
        self.token.push_str(s);
    }
    pub fn get_node(&self) -> &Node {
        return &self;
    }
    pub fn new(s:String, tt: TTYPES) -> Node {
        Node {
            token: s,
            token_type: tt,
            children: Vec::<Node>::new(),
        }
    }
    pub fn update_parent(&mut self, children_node: Node) {
        self.children.push(children_node);
    }

    pub fn get_children(&mut self) -> &mut Vec<Node>{
        return &mut self.children;
    }

    pub fn get_last_child(&mut self) -> &mut Node {
        let len = self.children.len();

        let c = &mut self.children[len-1];
        return c;
    }
}

fn main() {

}
