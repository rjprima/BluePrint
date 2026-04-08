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
                    ver: String::from(""), 
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

/* 
first match is destination, second match is origin. takes the origin "unpacks" the correct
component and then adds both to path as separate objects with ownership
*/
fn cd(name: String, locat: String, mut path: Vec<Window>) {
    let last_enum = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    match locat.as_str() {
        "component" => {
            match last_enum {
                Window::prj(mut x) => {
                    let mut temp = Component {
                        name: String::from("default"),
                        UDPs: vec![],
                        DBs: vec![],
                        dependencies: vec![]
                    };
                    for i in 0..x.components.len()-1 {
                        if x.components[i].name == locat {
                            temp = x.components.remove(i);
                            break;
                        }
                    }
                    path.push(Window::prj(x));
                    path.push(Window::cmp(temp));
                }
                Window::cmp(x) => {}
                _ => {println!("does not contain components");}
            }            
        }
        "UDP" => {
            match last_enum {
                Window::cmp(mut x) => {
                    let mut temp = UDP {
                        name: String::from("default"),
                        lang_type: String::from("default"),
                        language: String::from("default"),
                        functions: vec![],
                        class_vars: vec![],
                        structs: vec![],
                        dependencies: vec![]
                    };
                    for i in 0..x.UDPs.len()-1 {
                        if x.UDPs[i].name == locat {
                            temp = x.UDPs.remove(i);
                            break;
                        }
                    }
                    path.push(Window::cmp(x));
                    path.push(Window::userdef(temp));
                }
                Window::userdef(mut x) => {}
                Window::prj(mut x) => {
                    let mut temp = UDP {
                        name: String::from("default"),
                        lang_type: String::from("default"),
                        language: String::from("default"),
                        functions: vec![],
                        class_vars: vec![],
                        structs: vec![],
                        dependencies: vec![]
                    };
                    for i in 0..x.mains.len()-1 {
                        if x.mains[i].name == locat {
                            temp = x.mains.remove(i);
                            break;
                        }
                    }
                    path.push(Window::prj(x));
                    path.push(Window::userdef(temp));
                }
                _ => {println!("does not contain components");}
            }
        }
        "DB" => {
            match last_enum {
                Window::cmp(mut x) => {
                    let mut temp = DB {
                        name: String::from("default"),
                        manage_sys: String::from("default"),
                        data_sets: vec![],
                        dependencies: vec![],
                        src: String::from("default")
                    };
                    for i in 0..x.DBs.len()-1 {
                        if x.DBs[i].name == locat {
                            temp = x.DBs.remove(i);
                            break;
                        }
                    }
                    path.push(Window::cmp(x));
                    path.push(Window::database(temp));
                }
                _ => {println!("does not contain components");}
            }
        }
        "dependency" => {
            match last_enum {
                _ => {println!("does not contain components");}
            }
        }
        "function" => {
            match last_enum {
                Window::userdef(mut x) => {
                    let mut temp = Func {
                        name: String::from("default"),
                        pre_con: String::from("default"),
                        post_con: String::from("default"),
                        impl_details: String::from("default"),
                        params: vec![],
                        ret_val: String::from("default"),
                        dependencies: vec![],
                        description: String::from("default")
                    };
                    for i in 0..x.functions.len()-1 {
                        if x.functions[i].name == locat {
                            temp = x.functions.remove(i);
                            break;
                        }
                    }
                    path.push(Window::userdef(x));
                    path.push(Window::function(temp));
                }
                _ => {println!("does not contain components");}
            }
        }
        "dataset" => {
            match last_enum {
                Window::database(mut x) => {
                    let mut temp = DataSet {
                        ID: String::from("default"),
                        properties: vec![]
                    };
                    for i in 0..x.data_sets.len()-1 {
                        if x.data_sets[i].ID == locat {
                            temp = x.data_sets.remove(i);
                            break;
                        }
                    }
                    path.push(Window::database(x));
                    path.push(Window::dataset(temp));
                }
                _ => {println!("does not contain components");}
            }
        }
        "resource" => {
            match last_enum {
                _ => {println!("does not contain components");}
            }
        }
        _ => {}
    }
}

fn cd_back(mut path: Vec<Window>) {
    let current = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    let parent = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    match parent {
        Window::prj(mut x) => {
            match current {
                Window::cmp(y) => {
                    x.components.push(y);
                }
                Window::userdef(y) => {
                    x.mains.push(y);
                }
                _ => {}
            }
            path.push(Window::prj(x));
        }
        Window::cmp(mut x) => {
            match current {
                Window::userdef(y) => {
                    x.UDPs.push(y);
                }
                Window::database(y) => {
                    x.DBs.push(y);
                }
                _ => {}
            }
            path.push(Window::cmp(x));
        }
        Window::userdef(mut x) => {
            match current {
                Window::function(y) => {
                    x.functions.push(y);
                }
                Window::depen(y) => {
                    x.dependencies.push(y);
                }
                _ => {}
            }
            path.push(Window::userdef(x));
        }
        Window::database(mut x) => {
            match current {
                Window::dataset(y) => {
                    x.data_sets.push(y);
                }
                Window::depen(y) => {
                    x.dependencies.push(y);
                }
                _ => {}
            }
            path.push(Window::database(x));
        }
        _ => {}
    }
}

fn edit(name: String, mut path: Vec<Window>) {

}

fn save(mut path: Vec<Window>) {

}