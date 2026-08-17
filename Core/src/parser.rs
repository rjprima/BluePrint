use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Clone)]
pub struct Dependency {
    pub src: String,
    pub src_type: String,
    pub com_interface: String,
    pub path: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Func {
    pub name: String,
    pub pre_con: String,
    pub post_con: String,
    pub impl_details: String,
    pub params: Vec<String>,
    pub ret_val: String,
    pub dependencies: Vec<Dependency>,
    pub description: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DataSet {
    pub ID: String,
    pub properties: Vec<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DB {
    pub name: String,
    pub manage_sys: String,
    pub data_sets: Vec<DataSet>,
    pub dependencies: Vec<Dependency>,
    pub src: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct rsrc {
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct UDP {
    pub name: String,
    pub lang_type: String,
    pub language: String,
    pub functions: Vec<Func>,
    pub class_vars: Vec<String>,
    pub structs: Vec<HashMap<String, String>>,
    pub dependencies: Vec<Dependency>
}

#[derive(Deserialize, Serialize)]
pub struct Component {
    pub name: String,
    pub UDPs: Vec<UDP>,
    pub DBs: Vec<DB>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub ver: String,
    pub name: String,
    pub standards: Vec<String>,
    pub system_group: String,
    pub components: Vec<Component>,
    pub mains: Vec<UDP>,
    pub tools: Vec<String>
}

pub struct ProjectPromise {
    pub package: Project,
    pub status: String
}

pub enum Window {
    prj(Project),
    cmp(Component),
    userdef(UDP),
    resource(rsrc),
    database(DB),
    dataset(DataSet),
    function(Func),
    depen(Dependency),
    default(String)
}

pub fn read(data: &File)->ProjectPromise {
    let reader = BufReader::new(data);
    match serde_json::from_reader(reader) {
        Ok(prj) => {
            return ProjectPromise {package: prj, status: String::from("Success")};
        }
        Err(e) => {
            return ProjectPromise {
                package: Project {
                    ver: String::from(""), 
                    name: String::from(""), 
                    standards: vec![], 
                    system_group: String::from(""), 
                    components: vec![], 
                    mains: vec![], 
                    tools: vec![]}, 
                status: String::from("Failure")};
        }
    }
}

pub fn write(locat: &File, prj: Project) {
    let reader = BufWriter::new(locat);
    match serde_json::to_writer(reader, &prj) {
        Ok(e) => {
            
        }
        Err(e) => {
            
        }
    }
}

/* 
first match is destination, second match is origin. takes the origin "unpacks" the correct
component and then adds both to path as separate objects with ownership
*/
pub fn cd(name: String, locat: String, path: &mut Vec<Window>) {
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
                        if x.components[i].name == name {
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
                        if x.UDPs[i].name == name {
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
                        if x.mains[i].name == name {
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
                        if x.DBs[i].name == name {
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
                        if x.functions[i].name == name {
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
                        if x.data_sets[i].ID == name {
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

pub fn cd_back(path: &mut Vec<Window>) {
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

pub fn edit(field_name: &str, val: String, path: &mut Vec<Window>) {
    let current = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    match current {
        Window::prj(mut x) => {
            match field_name {
                "ver" => {x.ver = val;}
                "name" => {x.name = val;}
                "standards" => {}
                "system_group" => {x.system_group = val;}
                _ => {}
            }
            path.push(Window::prj(x));
        }
        Window::cmp(mut x) => {
            match field_name {
                "name" => {x.name = val;}
                _ => {}
            }
            path.push(Window::cmp(x));
        }
        Window::userdef(mut x) => {
            match field_name {
                "name" => {x.name = val;}
                "lang_type" => {x.lang_type = val;}
                "language" => {x.language = val;}
                _ => {}
            }
            path.push(Window::userdef(x));
        }
        Window::resource(mut x) => {
            match field_name {
                "name" => {x.name = val;}
                _ => {}
            }
            path.push(Window::resource(x));
        }
        Window::database(mut x) => {
            match field_name {
                "name" => {x.name = val;}
                "manage_sys" => {x.manage_sys = val;}
                "data_sets" => {}
                "dependencies" => {}
                "src" => {x.src = val;}
                _ => {}
            }
            path.push(Window::database(x));
        }
        Window::dataset(mut x) => {
            match field_name {
                "ID" => {x.ID = val;}
                "properties" => {}
                _ => {}
            }
            path.push(Window::dataset(x));
        }
        Window::function(mut x) => {
            match field_name {
                "name" => {x.name = val;}
                "pre_con" => {x.pre_con = val;}
                "post_con" => {x.post_con = val;}
                "impl_details" => {x.impl_details = val;}
                "ret_val" => {x.ret_val = val;}
                "description" => {x.description = val;}
                _ => {}
            }
            path.push(Window::function(x));
        }
        Window::depen(mut x) => {
            match field_name {
                "src" => {x.src = val;}
                "src_type" => {x.src_type = val;}
                "com_interface" => {x.com_interface = val;}
                _ => {}
            }
            path.push(Window::depen(x));
        }
        _ => {}
    }

}

pub fn save(path: &mut Vec<Window>, file: &File) {
    let length = path.len();
    let mut org_path: Vec<(String, String)> = vec![];
    let default = Window::default(String::from("unfound"));
    for i in 2..length {
        let current = match path.last() {
            Some(last) => last,
            None => &default
        };
        match current {
            Window::cmp(x) => {
                org_path.push((x.name.clone(), String::from("component")));
            }
            Window::userdef(x) => {
                org_path.push((x.name.clone(), String::from("UDP")));
            }
            Window::resource(x) => {
                org_path.push((x.name.clone(), String::from("resource")));
            }
            Window::database(x) => {
                org_path.push((x.name.clone(), String::from("DB")));
            }
            Window::dataset(x) => {
                org_path.push((x.ID.clone(), String::from("dataset")));
            }
            Window::function(x) => {
                org_path.push((x.name.clone(), String::from("function")));
            }
            Window::depen(x) => {
                org_path.push((String::from(""), String::from("dependency")));
            }
            _ => {}
        }
        cd_back(path);
    }
    let prj = match path.pop() {
            Some(last) => last,
            None => default
    };
    match prj {
        Window::prj(x) => {
            write(file, x);
        }
        _ => {print!("error")}
    }
    for i in 1..org_path.len() {
        let current = match org_path.pop() {
            Some(last) => last,
            None => (String::from(""), String::from(""))
        };
        cd(current.0, current.1, path);
    }
}

pub fn follow_path(path: &mut Vec<Window>, given_path: &mut Vec<(String, String)>) {
    for i in 1..given_path.len() {
        let current = match given_path.pop() {
            Some(last) => last,
            None => (String::from(""), String::from(""))
        };
        cd(current.0, current.1, path);
    }
}

pub fn add(path: &mut Vec<Window>, field_name: &str, init_val: String) {
    let curr = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    match curr {
        Window::prj(mut x) => {
            match field_name {
                "standards" => {
                    x.standards.push(init_val);
                }
                "components" => {
                    x.components.push(Component {
                        name: init_val,
                        UDPs: vec![],
                        DBs: vec![],
                        dependencies: vec![]
                    });
                }
                "tools" => {
                    x.tools.push(init_val);
                }
                _ => {}
            }
            path.push(Window::prj(x));
        }
        Window::cmp(mut x) => {
            match field_name {
                "UDPs" => {
                    x.UDPs.push(UDP {
                        name: init_val,
                        lang_type: String::from(""),
                        language: String::from(""),
                        functions: vec![],
                        class_vars: vec![],
                        structs: vec![],
                        dependencies: vec![]
                    })
                }
                "DBs" => {
                    x.DBs.push(DB {
                        name: init_val,
                        manage_sys: String::from(""),
                        data_sets: vec![],
                        dependencies: vec![],
                        src: String::from("")
                    })
                }
                _ => {}
            }
            path.push(Window::cmp(x));
        }
        Window::userdef(mut x) => {
            match field_name {
                "functions" => {
                    x.functions.push(Func {
                        name: init_val,
                        pre_con: String::from(""),
                        post_con: String::from(""),
                        impl_details: String::from(""),
                        params: vec![],
                        ret_val: String::from(""),
                        dependencies: vec![],
                        description: String::from("")
                    })
                }
                "class_vars" => {
                    x.class_vars.push(init_val);
                }
                "structs" => {
                    let mut temp = HashMap::new();
                    temp.insert(String::from("name"), init_val);
                    x.structs.push(temp);
                }
                _ => {}
            }
            path.push(Window::userdef(x));
        }
        Window::function(mut x) => {
            x.params.push(String::from(""));
            path.push(Window::function(x));
        }
        Window::database(mut x) => {
            x.data_sets.push(DataSet {
                ID: init_val,
                properties: vec![],
            });
            path.push(Window::database(x));
        }
        Window::dataset(mut x) => {
            x.properties.push(init_val);
            path.push(Window::dataset(x));
        }
        _ => {}
    }
}

pub fn remove(path: &mut Vec<Window>, field_name: &str, ID: usize) {
    let curr = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    match curr {
        Window::prj(mut x) => {
            match field_name {
                "standards" => {
                    x.standards.remove(ID);
                }
                "components" => {
                    x.components.remove(ID);
                }
                "tools" => {
                    x.tools.remove(ID);
                }
                _ => {}
            }
            path.push(Window::prj(x));
        }
        Window::cmp(mut x) => {
            match field_name {
                "UDPs" => {
                    x.UDPs.remove(ID);
                }
                "DBs" => {
                    x.DBs.remove(ID);
                }
                _ => {}
            }
            path.push(Window::cmp(x));
        }
        Window::userdef(mut x) => {
            match field_name {
                "functions" => {
                    x.functions.remove(ID);
                }
                "class_vars" => {
                    x.class_vars.remove(ID);
                }
                "structs" => {
                    x.structs.remove(ID);;
                }
                _ => {}
            }
            path.push(Window::userdef(x));
        }
        Window::function(mut x) => {
            x.params.remove(ID);;
            path.push(Window::function(x));
        }
        Window::database(mut x) => {
            x.data_sets.remove(ID);
            path.push(Window::database(x));
        }
        Window::dataset(mut x) => {
            x.properties.remove(ID);;
            path.push(Window::dataset(x));
        }
        _ => {}
    }
}

pub fn strip(path: &mut Vec<Window>) -> Window {
    let curr = match path.pop() {
        Some(last) => last,
        None => Window::default(String::from("unfound"))
    };
    let mut ret = Window::default(String::from(""));
    match curr {
        Window::prj(x) => {
            let mut temp = Project {
                name: x.name.clone(),
                ver: x.ver.clone(),
                standards: x.standards.clone(),
                system_group: x.system_group.clone(),
                components: vec![],
                mains: vec![],
                tools: x.tools.clone(),
            };
            for i in 0..x.components.len() {
                temp.components.push(Component {
                    name: x.components[i].name.clone(),
                    UDPs: vec![],
                    DBs: vec![],
                    dependencies: x.components[i].dependencies.clone()
                });
            }
            for i in 0..x.mains.len() {
                temp.mains.push(UDP {
                    name: x.mains[i].name.clone(),
                    lang_type: String::from(""),
                    language: String::from(""),
                    functions: vec![],
                    class_vars: vec![],
                    structs: vec![],
                    dependencies: x.mains[i].dependencies.clone()
                });
            }
            path.push(Window::prj(x));
            ret = Window::prj(temp);
        }
        Window::cmp(x) => {
            let mut temp = Component {
                name: x.name.clone(),
                UDPs: vec![],
                DBs: vec![],
                dependencies: x.dependencies.clone()
            };
            for i in 0..x.UDPs.len() {
                temp.UDPs.push(UDP {
                    name: x.UDPs[i].name.clone(),
                    lang_type: String::from(""),
                    language: String::from(""),
                    functions: vec![],
                    class_vars: vec![],
                    structs: vec![],
                    dependencies: x.UDPs[i].dependencies.clone()
                });
            }
            for i in 0..x.DBs.len() {
                temp.DBs.push(DB {
                    name: x.DBs[i].name.clone(),
                    src: String::from(""),
                    manage_sys: x.DBs[i].manage_sys.clone(),
                    data_sets: vec![],
                    dependencies: x.DBs[i].dependencies.clone()
                });
            }
            path.push(Window::cmp(x));
            ret = Window::cmp(temp);
        }
        Window::userdef(x) => {
            let mut temp = UDP {
                name: x.name.clone(),
                lang_type: x.lang_type.clone(),
                language: x.language.clone(),
                functions: x.functions.clone(),
                class_vars: x.class_vars.clone(),
                structs: x.structs.clone(),
                dependencies: x.dependencies.clone()
            };
            path.push(Window::userdef(x));
            ret = Window::userdef(temp);
        }
        Window::database(x) => {
            let mut temp = DB {
                name: x.name.clone(),
                manage_sys: x.manage_sys.clone(),
                data_sets: x.data_sets.clone(),
                dependencies: x.dependencies.clone(),
                src: x.src.clone()
            };
            path.push(Window::database(x));
            ret = Window::database(temp);
        }
        _ => {}
    }
    return ret;
}