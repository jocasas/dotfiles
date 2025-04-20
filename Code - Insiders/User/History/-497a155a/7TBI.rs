// src/app/core.rs

// file manage
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Read;

use crate::models::project::Project;

// se agrega aca en el vec y en el modelo para poder serializarlo a
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectList {
    pub filename: String,
    pub projects: Vec<Project>,
}

#[allow(dead_code)]
impl ProjectList {
    // Crea nuevo archivo JSON vacío
    pub fn new_file(filename: &str) -> ProjectList {
        let list = ProjectList {
            filename: filename.to_string(),
            projects: Vec::new(),
        };
        list.save_file();
        list
    }

    // Carga desde archivo JSON
    pub fn load_file(filename: &str) -> ProjectList {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => {
                return ProjectList {
                    filename: filename.to_string(),
                    projects: Vec::new(),
                };
            }
        };

        let mut json_string = String::new();
        file.read_to_string(&mut json_string).unwrap();

        let mut list: ProjectList = serde_json::from_str(&json_string).unwrap_or(ProjectList {
            filename: filename.to_string(),
            projects: Vec::new(),
        });

        list.filename = filename.to_string(); // asegura que se setee el filename tras deserializar
        list
    }

    // Guarda al archivo JSON
    pub fn save_file(&self) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.filename)
            .unwrap();

        serde_json::to_writer(file, &self).unwrap();
    }

    // Agrega proyecto
    pub fn save_project(&mut self, project: Project) {
        self.projects.push(project);
        self.save_file();
    }


    // Elimina proyecto por nombre (o cambia por ID si es necesario)
    // Params pid (project id)
    pub fn delete_project(&mut self, id: u8) {
        self.projects.retain(|p| p.id != id);
        self.save_file();
    }


    // Muestra proyectos
    pub fn show_projects(&self) {
        for project in &self.projects {
            println!("{:?}", project);
        }
    }

}
