//! Code for initialization of LaTeX projects.

use std::env;
use std::fs::{create_dir_all, File};
use crate::template;
use promptuity::prompts::Input;
use promptuity::themes::FancyTheme;
use promptuity::{Promptuity, Term};
use std::path::PathBuf;
use crate::template::TemplateConfiguration;

pub(crate) fn cli_init(template: Option<PathBuf>) -> Result<(), promptuity::Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.with_intro("LaTeX project initialization").begin()?;

    if let Some(template_dir) = template {
        p.info(format!("Template directory located at {}; attempting to read configuration.", template_dir.display()))?;
        let mut template = template_dir.clone();
        template.push("template.templex.toml");
        let cfg: TemplateConfiguration;
        match template::read_template(&template) {
            Ok(config) => {
                p.success("Template successfully read.")?;
                p.info("The following files will be copied:")?;
                config.files_to_copy.iter().for_each(|x| p.info(format!("--> {}", x.display())).unwrap());
                p.info("The following folders will be created:")?;
                config.folders_to_create.iter().for_each(|x| p.info(format!("--> {}", x.display())).unwrap());
                cfg = config;
            }
            Err(err) => {
                p.error(format!("Failed to read template: {}", err))?;
                return p.with_outro("Error encountered; terminating.").finish()
            }
        }
        p.log("")?;
        p.info("To cancel this operation, press Ctrl+C. Otherwise, answer the following questions.")?;
        p.log("")?;
        let folder_name = p.prompt(Input::new("What would you like your project's folder name to be?").with_placeholder("new-project"))?;

        let mut proj_dir = env::current_dir()?;
        proj_dir.push(folder_name + "/");
        std::fs::create_dir(&proj_dir)?;
        p.success(format!("Directory {} created.", proj_dir.display()))?;
        for dir in cfg.folders_to_create {
            let mut x = proj_dir.clone();
            x.push(dir);
            create_dir_all(&x)?;
            p.success(format!("Directory {} created.", x.display()))?;
        }
        for file in cfg.files_to_copy {
            let mut x = proj_dir.clone();
            x.push(&file);
            let mut project_copy = File::create(x)?;
            let mut y = template_dir.clone();
            y.push(&file);
            let mut template_copy = File::open(y)?;
            std::io::copy(&mut template_copy, &mut project_copy)?;
            p.success(format!("File {} copied over.", file.display()))?;
        }
        p.log("")?;
        p.with_outro(format!("Project created in '{}'.", proj_dir.display())).finish()?;
    } else {
        return p.with_outro("No such template exists; terminating.").finish()
    }
    Ok(())
}