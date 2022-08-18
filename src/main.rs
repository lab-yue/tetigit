use clap::Parser;
use git2::{Error, ObjectType, Repository, TreeWalkMode, TreeWalkResult};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Author to search
    #[clap(short, long, value_parser, value_delimiter = ',')]
    author: Vec<String>,
    /// Repository path
    #[clap(short, long, value_parser)]
    path: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let authors = HashSet::<String>::from_iter(args.author);
    let repo = Repository::init(Path::new(&args.path.unwrap_or(".".to_owned())))?;
    let commit_id = repo.head()?.target().ok_or(Error::from_str("No HEAD?"))?;

    let mut files: Vec<PathBuf> = vec![];
    repo.find_commit(commit_id)?
        .tree()?
        .walk(TreeWalkMode::PreOrder, |parent, entry| {
            if let Some(name) = entry.name()  && entry.kind() != Some(ObjectType::Tree){
                files.push([parent, name].iter().collect::<PathBuf>());
            }
            TreeWalkResult::Ok
        })?;

    let author_count: i32 = files
        .iter()
        .map(|path_buf| {
            repo.blame_file(&path_buf.as_path(), None)
                .ok()
                .expect(&format!("blame {:?} failed", path_buf.as_path()))
                .iter()
                .find(|hunk| {
                    hunk.final_signature()
                        .name()
                        .map(|name| authors.contains(name))
                        .unwrap_or(false)
                })
                .is_some() as i32
        })
        .sum();

    println!(
        "{} / {} files ({:.2}%) are touched by {}",
        author_count,
        files.len(),
        author_count as f32 / files.len() as f32 * 100.,
        authors.into_iter().collect::<Vec<String>>().join(",")
    );
    Ok(())
}
