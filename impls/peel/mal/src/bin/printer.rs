use crate::reader;
use crate::types;

pub fn print_preorder(node:&mut types::Node, input:&mut String, mut add_space: bool) {
    let len = node.get_children().len();

    // TODO: REPLACE
    // if node.get_token() == "(with-meta " {
    //     input.push_str(&node.get_token());
    //     add_space = false;

    //     let mut c = &mut node.get_children()[2];
    //     print_preorder(c, input, add_space);

    //     c = &mut node.get_children()[3];
    //     print_preorder(c, input, add_space);

    //     input.push_str(" ");
    //     c = &mut node.get_children()[0];
    //     print_preorder(c, input, add_space);

    //     c = &mut node.get_children()[1];
    //     print_preorder(c, input, add_space);
    //     return;
    // }

    if node.get_token() != "" {
        if add_space == true && node.get_token() != ")" && node.get_token() != "]" && node.get_token() != "}" {
            input.push_str(" ");
        }

        let current_token_type = node.get_token_type();
        match current_token_type {
            types::TTYPES::comment => {

            }
            _ => {
                input.push_str(&node.get_token());
            }
        }


        // let current_token_type = node.get_token_type();
        // match current_token_type {
        //     types::TTYPES::list => {
        //         println!("token: {}, type: list", node.get_token());
        //     }
        //     types::TTYPES::symbol => {
        //         println!("token: {}, type: symbol", node.get_token());
        //     }
        //     types::TTYPES::integer => {
        //         println!("token: {}, type: integer", node.get_token());
        //     }
        //     types::TTYPES::string => {
        //         println!("token: {}, type: string", node.get_token());
        //     }
        //     types::TTYPES::comment => {
        //         println!("token: {}, type: comment", node.get_token());
        //     }
        //     types::TTYPES::twoSpecialChar => {
        //         println!("token: {}, type: twoSpecialChar", node.get_token());
        //     }
        //     types::TTYPES::specialChar => {
        //         println!("token: {}, type: specialChar", node.get_token());
        //     }
        //     types::TTYPES::identifier => {
        //         println!("token: {}, type: identifier", node.get_token());
        //     }
        //     types::TTYPES::keyword => {
        //         println!("token: {}, type: keyword", node.get_token());
        //     }
        //     types::TTYPES::end => {
        //         println!("token: {}, type: end", node.get_token());
        //     }
        //     types::TTYPES::vector => {
        //         println!("token: {}, type: vector", node.get_token());
        //     }
        //     types::TTYPES::hashmap => {
        //         println!("token: {}, type: hashmap", node.get_token());
        //     }
        //     _ => {}
        // }
    }

    if len != 0 {
        // println!("Children");
        let mut c = &mut node.get_children()[0];

        let mut i = 0;

        while i < len {
            if i == 0 {
                add_space = false;
            }
            else {
                add_space = true;
            }

            print_preorder(c, input, add_space);

            i += 1;
            if i != len {
                c = &mut node.get_children()[i];
            }
        }
        // println!("Back to Parent");
    }
}

fn main () {

}