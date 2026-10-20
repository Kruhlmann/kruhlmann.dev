use std::{collections::BTreeMap, fs::DirEntry, path::Path};

#[derive(Clone, Debug)]
pub struct TexContextEducation {
    pub graduation_year: u16,
    pub institution: String,
    pub location: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct TexContextExperience {
    pub year_start: u16,
    pub year_stop: u16,
    pub position: String,
    pub company: String,
    pub assignments: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct TexContext {
    pub profile: String,
    pub education: Vec<TexContextEducation>,
    pub experience: Vec<TexContextExperience>,
    pub languages: BTreeMap<String, String>,
    pub methods: Vec<String>,
    pub tools: BTreeMap<String, Vec<String>>,
    pub vars: BTreeMap<String, String>,
}

#[derive(thiserror::Error, Debug)]
pub enum TexContextLoadError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("parse error")]
    ParseError(#[from] std::num::ParseIntError),
}

fn file_to_name_content_map(file: &DirEntry) -> Result<(String, String), std::io::Error> {
    let name = file
        .path()
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| std::io::Error::other("Invalid filename"))?
        .to_owned();
    let content = std::fs::read_to_string(file.path())
        .inspect_err(|_| tracing::error!(?file, "read file to name content map"))?
        .trim()
        .to_owned();
    Ok((name, content))
}

fn dir_to_file_name_content_map<P: AsRef<Path> + std::fmt::Debug>(
    dir: P,
) -> Result<BTreeMap<String, String>, TexContextLoadError> {
    Ok(read_dir_contents_sorted(dir.as_ref())
        .inspect_err(|_| tracing::error!(?dir, "read dir to name content map"))?
        .iter()
        .filter(|node| node.file_type().is_ok_and(|kind| kind.is_file()))
        .map(file_to_name_content_map)
        .collect::<Result<BTreeMap<String, String>, _>>()?)
}

fn read_fs_content_observable<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> Result<String, TexContextLoadError> {
    let res = std::fs::read_to_string(&path)
        .inspect_err(|_| tracing::error!(?path, "read file content"))?;
    Ok(res)
}

fn read_dir_contents_sorted<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> Result<Vec<DirEntry>, TexContextLoadError> {
    let mut items = std::fs::read_dir(&path)
        .inspect_err(|_| tracing::error!(?path, "read dir content"))?
        .collect::<Result<Vec<_>, std::io::Error>>()
        .inspect_err(|_| tracing::error!(?path, "collect dir content"))?;
    items.sort_by_key(|entry| entry.file_name());
    Ok(items)
}

impl TexContext {
    pub fn var(&self, key: &str) -> &str {
        self.vars.get(key).map(String::as_str).unwrap_or("unknown")
    }

    pub fn load_from_directory<P: AsRef<Path>>(directory: P) -> Result<Self, TexContextLoadError> {
        let include_dir = directory.as_ref().join("include").join("tex");
        let profile_path = include_dir.join("profile.tex");
        let education_dir = include_dir.join("education");
        let experience_dir = include_dir.join("experience");
        let languages_dir = include_dir.join("languages");
        let vars_dir = include_dir.join("vars");
        let methods_dir = include_dir.join("methods");
        let tools_dir = include_dir.join("tools");

        let profile = read_fs_content_observable(profile_path)?;
        let education = read_dir_contents_sorted(education_dir)
            .inspect_err(|_| tracing::error!("read education directory"))?
            .iter()
            .filter(|node| node.file_type().is_ok_and(|kind| kind.is_dir()))
            .map(|dir| -> Result<TexContextEducation, TexContextLoadError> {
                let graduation_year: u16 =
                    read_fs_content_observable(dir.path().join("graduation_year.tex"))?
                        .trim()
                        .parse()?;
                let location: String = read_fs_content_observable(dir.path().join("location.tex"))?
                    .trim()
                    .to_string();
                let institution: String =
                    read_fs_content_observable(dir.path().join("institution.tex"))?
                        .trim()
                        .to_string();
                let title: String = read_fs_content_observable(dir.path().join("title.tex"))?
                    .trim()
                    .to_string();
                Ok(TexContextEducation {
                    graduation_year,
                    institution,
                    location,
                    title,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let experience = read_dir_contents_sorted(experience_dir)
            .inspect_err(|_| tracing::error!("read experience directory"))?
            .iter()
            .filter(|node| node.file_type().is_ok_and(|kind| kind.is_dir()))
            .map(|dir| -> Result<TexContextExperience, TexContextLoadError> {
                let company: String = read_fs_content_observable(dir.path().join("company.tex"))?
                    .trim()
                    .to_string();
                let position: String = read_fs_content_observable(dir.path().join("position.tex"))?
                    .trim()
                    .to_string();
                let year_start: u16 =
                    read_fs_content_observable(dir.path().join("year_start.tex"))?
                        .trim()
                        .parse()?;
                let year_stop: u16 = read_fs_content_observable(dir.path().join("year_stop.tex"))?
                    .trim()
                    .parse()?;
                let assignments = dir_to_file_name_content_map(dir.path().join("assignments"))?;
                Ok(TexContextExperience {
                    year_start,
                    year_stop,
                    position,
                    company,
                    assignments,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let languages = dir_to_file_name_content_map(languages_dir)?;
        let vars = dir_to_file_name_content_map(vars_dir)?;
        let methods = dir_to_file_name_content_map(methods_dir)?
            .values()
            .flat_map(|values| {
                values
                    .split(",")
                    .map(|s| s.trim())
                    .map(str::to_owned)
                    .collect::<Vec<String>>()
            })
            .collect();
        let tools = dir_to_file_name_content_map(tools_dir)?
            .iter()
            .map(|(category, values)| {
                let values_vec = values
                    .split(",")
                    .map(|s| s.trim())
                    .map(str::to_owned)
                    .collect::<Vec<String>>();
                (category.to_owned(), values_vec)
            })
            .collect::<BTreeMap<String, Vec<String>>>();

        Ok(Self {
            profile,
            education,
            experience,
            languages,
            vars,
            methods,
            tools,
        })
    }
}
