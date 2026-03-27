use std::fs::File;
use std::collections::HashMap;

struct Dependency {
    src: String,
    src_type: String,
    com_interface: String
}

struct Func {
    name: String,
    pre_con: String,
    post_con: String,
    impl_details: String,
    params: Vec<String>,
    ret_val: String,
    dependencies: Vec<Dependency>,
    description: String
}

struct DataSet {
    ID: String,
    properties: Vec<String>
}

struct DB {
    name: String,
    manage_sys: String,
    data_sets: Vec<DataSet>,
    dependencies: Vec<Dependency>,
    src: String
}

struct rsrc {
    name: String
}

struct UDP {
    name: String,
    lang_type: String,
    language: String,
    functions: Vec<Func>,
    class_vars: Vec<String>,
    structs: Vec<HashMap<String, String>>,
    dependencies: Vec<Dependency>
}

struct Component {
    name: String,
    UDPs: Vec<UDP>,
    DBs: Vec<DB>,
    dependencies: Vec<Dependency>,
}

struct project {
    ver: String,
    name: String,
    standards: Vec<String>,
    system_group: String,
    components: Vec<Component>,
    mains: Vec<UDP>,
    tools: Vec<String>
}

fn read() {

}

fn write() {

}