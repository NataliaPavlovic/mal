use crate::types;
use crate::printer;
use std::collections::BTreeMap;

use regex::Regex;
use regex::RegexSet;

pub struct Reader {
    tokens: Vec<types::Node>,
    position: usize,
}

impl Reader {
    // next returns the token at the current position and increments the position.
    fn next(&mut self) -> &types::Node {
        self.position += 1;
        // println!("position: {}", self.position);
        &self.tokens[self.position -1]
    }

    //peek just returns the token at the current position.
    fn peek(&mut self) -> &types::Node  {
        // println!("tokens: {}", self.tokens[self.position].get_token());
        &self.tokens[self.position]
    }

    fn valid_position(&mut self) -> bool {
        if self.position >= self.tokens.len() {
            println!("EOF");
            return false;
        }
        else {
            return true;
        }
    }
}

pub fn read_str(input: &str) -> types::Node {
    let mut reader = Reader {
        tokens: tokenize(input),
        position: 0,
    };

    let mut node = types::Node::new(String::from(""), types::TTYPES::uninit);

    let mut map = BTreeMap::new();
    map.insert("~@", "(splice-unquote ");
    map.insert("'", "(quote ");
    map.insert("~", "(unquote ");
    map.insert("`", "(quasiquote ");
    map.insert("@", "(deref ");
    map.insert("^", "(with-meta ");

    read_form(&mut reader, &mut node, &map);

    return node;
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn remove_comma(s: &mut String) {
    s.retain(|c| !(c == ','));
}

// take string and return vec of tokens
fn tokenize(input: &str) -> Vec<types::Node>{
    let mut tokens_vec = Vec::<types::Node>::new();

    let set = RegexSet::new(&[
        r#"[\s,]*(~@)"#,
        r#"[\s,]*([\[\]{}()'`~^@])"#,
        r#"[\s,]*("(?:\\.|[^\\"])*"?)"#,
        r#"[\s,]*(;.*)"#,
        r#"[\s,]*([^\s\[\]{}('"`,;)]*)"#,
    ]).unwrap();

    for mat in Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap().find_iter(input) {
        let mut s = mat.as_str().to_string();
        // println!("{}", s);
        if s.chars().next().unwrap() != '\"' && s.chars().next().unwrap() != ';' {
            remove_whitespace(&mut s);
            remove_comma(&mut s);
        }

        let matches = set.matches(&s);

        if s!= "" {
            let c = s.chars().next().unwrap();

            // Comment
            if matches.matched(3) && c == ';' {
                // println!("token:{} type:3", s);
                let node = types::Node::new(s, types::TTYPES::comment);
                tokens_vec.push(node);
            }
            // String
            else if matches.matched(2) && c == '"'{
                // println!("token:{} type:2", s);
                let node = types::Node::new(s, types::TTYPES::string);
                tokens_vec.push(node);
            }
            // Two Special Character
            else if matches.matched(0) {
                // println!("token:{} type:0", s);
                let node = types::Node::new(s, types::TTYPES::twoSpecialChar);
                tokens_vec.push(node);
            }
            // Special Character
            else if matches.matched(1) {
                // println!("token:{} type:1", s);
                let node = types::Node::new(s, types::TTYPES::specialChar);
                tokens_vec.push(node);
            }
            // Non Special Characters
            else if matches.matched(4) {
                // println!("token:{} type:4", s);
                let node = types::Node::new(s, types::TTYPES::uninit);
                tokens_vec.push(node);
            }
            else {
                println!("Error no regex match");
                // Todo break
            }
        }
    }
    return tokens_vec;
}

fn read_form(reader: &mut Reader, parent_node: &mut types::Node, map: &BTreeMap<&str, &str>) {
    let mut current_token = reader.peek().get_token();

    if current_token == "(" {
        let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::list);
        parent_node.update_parent(child_node);
        read_list(reader, parent_node.get_last_child(), map);
        let mut child_node = types::Node::new(")".to_string(), types::TTYPES::end);
        parent_node.update_parent(child_node);
    }
    else if current_token == "[" {
        let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::vector);
        parent_node.update_parent(child_node);
        read_list(reader, parent_node.get_last_child(), map);
        let mut child_node = types::Node::new("]".to_string(), types::TTYPES::end);
        parent_node.update_parent(child_node);
    }
    else if current_token == "{" {
        let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::hashmap);
        parent_node.update_parent(child_node);
        read_list(reader, parent_node.get_last_child(), map);
        let mut child_node = types::Node::new("}".to_string(), types::TTYPES::end);
        parent_node.update_parent(child_node);
    }
    else {
        read_atom(reader, parent_node, map);
    }
}

fn read_list(reader: &mut Reader,  parent_node: &mut types::Node, map: &BTreeMap<&str, &str>) {
    if reader.valid_position() == false {
        return;
    }
    let mut current_token = reader.next().get_token();
    if reader.valid_position() == false {
        return;
    }
    let mut next_token = reader.peek().get_token();

    let mut s = "";

    match parent_node.get_token_type() {
        types::TTYPES::list => {
            s = ")";
        }
        types::TTYPES::vector => {
            s = "]";
        }
        types::TTYPES::hashmap => {
            s = "}";
        }
        _ => {}
    }

    while next_token != s {
        // println!("{}", next_token);
        if next_token == "" {
            println!("EOF");
            break;
        }
        read_form(reader, parent_node, &map);
        if reader.valid_position() == false {
            break;
        }
        current_token = reader.next().get_token();
        if reader.valid_position() == false {
            break;
        }
        next_token = reader.peek().get_token();
    }
}

fn replace_special(reader: &mut Reader, parent_node: &mut types::Node, map: &BTreeMap<&str, &str>, current_token_type: types::TTYPES) {
    let current_token = reader.peek().get_token();

    let replacement_token = &map[current_token];

    let mut child_node = types::Node::new(replacement_token.to_string(), current_token_type);
    parent_node.update_parent(child_node);
    reader.next();

    if replacement_token.to_string() == "(with-meta " {
        read_form(reader, parent_node.get_last_child(), map);
        reader.next();
        read_form(reader, parent_node.get_last_child(), map);

        let children = parent_node.get_last_child().get_children();
        // Swap child 0 and 2
        let temp1 = children[0].clone();
        children[0] = children[2].clone();
        children[2] = temp1.clone();

        // Swap child 1 and 3
        let temp2 = children[1].clone();
        children[1] = children[3].clone();
        children[3] = temp2.clone();
    }
    else {
        read_form(reader, parent_node.get_last_child(), map);
    }
    let mut child_node = types::Node::new(")".to_string(), types::TTYPES::end);
    parent_node.update_parent(child_node);
}

fn read_atom(reader: &mut Reader,  parent_node: &mut types::Node, map: &BTreeMap<&str, &str>) {
    let current_token_type = reader.peek().get_token_type();

    match current_token_type {
        types::TTYPES::twoSpecialChar => {
            replace_special(reader, parent_node, map, types::TTYPES::twoSpecialChar)
        }
        types::TTYPES::specialChar => {
            replace_special(reader, parent_node, map, types::TTYPES::specialChar)
        }
        types::TTYPES::uninit => {
            let current_token = reader.peek().get_token();

            let c = current_token.chars().next().unwrap();

            // keyword
            if c == ':' {
                let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::keyword);
                parent_node.update_parent(child_node);
            }
            else if c.is_numeric() {
                let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::integer);
                parent_node.update_parent(child_node);
            }
            else if c.is_alphabetic() || c == '_' {
                let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::identifier);
                parent_node.update_parent(child_node);
            }
            else {
                let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::symbol);
                parent_node.update_parent(child_node);
            }
        }
        types::TTYPES::comment => {
            let current_token = reader.peek().get_token();

            let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::comment);
            parent_node.update_parent(child_node);
        }
        types::TTYPES::string => {
            let current_token = reader.peek().get_token();

            // Check if string is balanced

            let len = current_token.len();

            let c = current_token.chars().last().unwrap();

            if c != '\"' {
                println!("EOF");
                return;
            }

            let mut child_node = types::Node::new(current_token.to_string(), types::TTYPES::string);
            parent_node.update_parent(child_node);
        }
        _ => {}
    }
}

fn main() {

}
