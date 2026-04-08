use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Dependency {
    pub src: String,
    pub src_type: String,
    pub com_interface: String
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct DataSet {
    pub ID: String,
    pub properties: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct DB {
    pub name: String,
    pub manage_sys: String,
    pub data_sets: Vec<DataSet>,
    pub dependencies: Vec<Dependency>,
    pub src: String
}

#[derive(Deserialize, Serialize)]
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
    package: Project,
    status: String
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

pub fn read(data: File)->ProjectPromise {
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

pub fn write(locat: File, prj: Project) {
    let reader = BufWriter::new(locat);
    match serde_json::to_writer(reader, &prj) {
        Ok(e) => {
            
        }
        Err(e) => {
            
        }
    }
}