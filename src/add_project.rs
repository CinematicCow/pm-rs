use crate::{Editors, Error, Project};
use clap::Result;
use std::process::Command;
use std::{env, fs, path};

pub fn add(name: String, editor: Editors, path: path::PathBuf) -> Result<bool, Error> {
    let db_data =
        fs::read_to_string(path::PathBuf::from(env::var("HOME").unwrap()).join(".pm/db.json"))?;

    let mut parsed: Vec<Project> = serde_json::from_str(&db_data)?;

    let project = Project { name, editor, path };

    parsed.push(project);
    fs::write(
        path::PathBuf::from(env::var("HOME").unwrap()).join(".pm/db.json"),
        serde_json::to_vec(&parsed)?,
    )?;

    Ok(true)
}

pub fn open(name: String) -> Result<(), Error> {
    let db_data =
        fs::read_to_string(path::PathBuf::from(env::var("HOME").unwrap()).join(".pm/db.json"))?;

    let parsed: Vec<Project> = serde_json::from_str(&db_data)?;

    let project = parsed.iter().find(|project| project.name == name);

    match project {
        Some(proj) => {
            // let srr = proj.editor.as_ref();
            let editor: &str = match proj.editor {
                Editors::Vscode => "nano",
                Editors::Vim => "nvim",
            };
            Command::new(editor)
                .arg(&proj.path)
                .status()
                .expect("Something went wrong, please try again");
            // .current_dir(&_data.path.as_os_str())
        }
        None => {
            println!("not found")
        }
    }

    Ok(())
}
