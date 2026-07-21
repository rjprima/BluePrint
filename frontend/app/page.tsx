'use client'

import Image from "next/image";
import Link from "next/link";
import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect, InputEventHandler } from 'react';
import path from 'react';

export default function Home() {
  const path = require('path');
  const [projects, setProjects] = useState<String[]>([]);
  const [projectName, setProjectName] = useState<String>("");
  const [newPath, setNewPath] = useState<String>("");
  useEffect(() => {invoke<String[]>("get_file_paths").then((ret => {setProjects(ret);}));}, [])

  function createNewProject(project_name:String, path:String) {
    invoke<boolean>("create_file", {projectName: project_name, path: path}).then((code) => {
      if (code) {
        console.log("successfully created file")
        setProjects([...projects, `${path}\\${project_name}.txt`]);
      }
      else {
        console.log("file already exists or path is invalid")
      }
    });
  }

  const handleProjectName = (e:React.ChangeEvent<HTMLInputElement>) => {
    setProjectName(e.target.value);
  }

  const handleNewPath = (e:React.ChangeEvent<HTMLInputElement>) => {
    setNewPath(e.target.value);
  }

  return (
    <div>
      <div className="border border-black rounded flex flex-col relative w-[30vw] left-[35vw] top-[20vh] min-h-[60vh] justify-center">
        {
          projects.length === 0 ? (<div className="w-full h-full text-center text-4xl">
            No Files Available
          </div>) : 
          (projects.map((x) => <div className="border-b border-black h-[4vh]">{x.split(/[/\\]/).pop()!.replace(".txt", "")}</div>))
        }
        <Link href="" className="">

        </Link>
      </div>
      <input type="text" onChange={handleProjectName} placeholder="Project Name"></input>
      <input type="text" onChange={handleNewPath} placeholder="File Path"></input>
      <button onClick={() => createNewProject(projectName, newPath)} className="hover:text-red-600">New Project</button>
    </div>
  );
}
