pub mod parser;
use std::fs::File;
use parser::*;

/*match next.as_str() {
            "component" => {}
            "UDP" => {}
            "DB" => {}
            "dependency" => {}
            "function" => {}
            "dataset" => {}
            "resource" => {}
            _ => {}
        } */

fn main() {
    let mut ex = Project {
                    ver: String::from("1.0"), 
                    name: String::from(""), 
                    standards: vec![], 
                    system_group: String::from(""), 
                    components: vec![], 
                    mains: vec![], 
                    tools: vec![]};
    let x = File::create("base_template.txt");
    /*match x {
        Ok(target)=> {
            write(target, ex)
        }
        Err(e)=> {
            println!("error");
        }
    }*/
}