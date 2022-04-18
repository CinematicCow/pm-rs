use ::clap::{Parser, Subcommand};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::{env, fs, io, path};
use thiserror::Error;
mod add_project;
mod init;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Parser)]
#[clap(version, about, author)]
#[clap(propagate_version = true)]
struct Args {
    /// Commands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the manager. You only need to run this once
    Init,
    /// Add your project. Run 'help add' to know more
    Add {
        /// The path to your project root
        path: path::PathBuf,
        /// The name of your project
        #[clap(short)]
        name: String,
        /// The editor you would like to open the project with
        #[clap(short, arg_enum)]
        editor: Editors,
    },
    /// List your projects
    List,
    /// Opens up the specified project
    Open {
        /// Specify the name of the project
        name: String,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Project {
    name: String,
    path: path::PathBuf,
    editor: Editors,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum Editors {
    Vscode,
    Vim,
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Add { path, name, editor } => {
            // fs::canonicalize(path).unwrap(),
            match add_project::add(
                name.to_owned(),
                editor.to_owned(),
                fs::canonicalize(path).unwrap(),
            ) {
                Ok(_) => println!("Project added"),
                Err(reason) => println!("Could not add project\n{:?}", reason),
            };
        }
        Commands::Init => {
            init::initialize().unwrap();
        }
        Commands::List => {
            let projects_list = read_db().unwrap();
            if projects_list.len() > 0 {
                for project in projects_list {
                    println!("{}", project.name);
                }
            } else {
                println!("You don't have any projects added. Run 'help add' for more info")
            }
        }
        Commands::Open { name } => {
            add_project::open(name.to_owned()).ok();
        }
    }
}

fn read_db() -> Result<Vec<Project>, Error> {
    let db_data =
        fs::read_to_string(path::PathBuf::from(env::var("HOME").unwrap()).join(".pm/db.json"))?;

    let parsed: Vec<Project> = serde_json::from_str(&db_data)?;
    Ok(parsed)
}
