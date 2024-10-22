use std::{
    borrow::BorrowMut,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Ok;
use serde::Serialize;
use solp::api::Project;
use xml::{reader::XmlEvent, EventReader};

use crate::model::{Dep, Target, TargetKind};

use super::ConfigParser;

#[derive(Debug, Clone, Default)]
pub struct CSharpConfiguration {}

#[derive(Debug, Serialize)]
struct ProjectReference {
    name: String,
}

impl ConfigParser for CSharpConfiguration {
    fn parse(&self, path: PathBuf) -> anyhow::Result<Vec<Target>> {
        let file = std::fs::read_to_string(&path)?;
        let solution = solp::parse_str(&file).map_err(|e| anyhow::anyhow!(e))?;
        let solution = solution.iterate_projects().collect::<Vec<_>>();

        let parent_dir = std::fs::canonicalize(path.parent().unwrap())?;
        let mut targets = vec![];
        for project in &solution {
            let mut target = from_project_into_target(&parent_dir, project)?;

            if let Some(deps) = target.dependencies.borrow_mut() {
                deps.iter_mut().for_each(|d| {
                    let path_to_uri = d.name[parent_dir.to_str().unwrap().len() + 1..]
                        .to_string()
                        .replace('/', "\\");

                    if let Some(corresponding_proj) =
                        solution.iter().find(|p| p.path_or_uri.eq(&path_to_uri))
                    {
                        d.name = corresponding_proj.name.to_string();
                    }
                });
            }

            targets.push(target)
        }

        Ok(targets)
    }

    fn matches(&self, path: &Path) -> bool {
        match path.extension() {
            Some(e) => e.to_string_lossy().eq("sln"),
            None => false,
        }
    }
}

fn from_project_into_target(solution_path: &Path, project: &Project) -> anyhow::Result<Target> {
    let project_file_path = solution_path.join(project.path_or_uri.replace('\\', "/"));
    let dir_binding = project_file_path.clone();
    let canonical_dir = dir_binding.parent().unwrap();
    let file = std::fs::File::open(project_file_path)?;
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut project_references = vec![];
    let mut current_element = "".to_string();
    let mut target_kind = TargetKind::Library;

    for e in parser {
        match e? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                current_element.clone_from(&name.local_name);
                match current_element.as_str() {
                    "Project" => {
                        let sdk = attributes
                            .iter()
                            .find(|attr| attr.name.local_name == "Sdk")
                            .unwrap();

                        if sdk.value == "Microsoft.NET.Sdk.Web" {
                            target_kind = TargetKind::Binary;
                        }
                    }
                    "ProjectReference" => {
                        let include = attributes
                            .iter()
                            .find(|attr| attr.name.local_name == "Include")
                            .unwrap();
                        let non_canonical = canonical_dir.join(include.value.replace('\\', "/"));
                        let canonical = std::fs::canonicalize(non_canonical)?;
                        project_references.push(ProjectReference {
                            name: canonical.to_str().unwrap().to_string(),
                        })
                    }
                    &_ => {}
                }
            }
            XmlEvent::Characters(chars) => match chars.as_str() {
                "OutputType" => {
                    if chars == "Exe" {
                        target_kind = TargetKind::Binary
                    }
                }
                &_ => {}
            },
            XmlEvent::EndElement { .. } => current_element.clear(),
            _ => {}
        }
    }

    let mut target = Target::new(project.name.to_string(), target_kind, "".to_string());
    target.add_dependencies(
        project_references
            .into_iter()
            .map(|pr| Dep {
                name: pr.name,
                kind: TargetKind::Library,
            })
            .collect(),
    );

    Ok(target)
}
