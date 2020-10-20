use std::fs::File;
use std::path::Path;

use git2::Commit;
use git2::IndexAddOption;
use git2::ObjectType;
use git2::Repository;
use git2::Signature;

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn main() {
    let args: env::args().collect();
    let root = &args[1];
    let file_name = &args[2];
    let repo = Repository::init(root).unwrap();
    let mut index = repo.index().unwrap();

    File::create(file_name).unwrap();

    index.add_path(Path::new(file_name)).unwrap();
    index.write().unwrap();
    let lc_result = find_last_commit(&repo);

    let mut parent_commits: Vec<&Commit> = vec![];

    match lc_result.as_ref() {
        Ok(c) => parent_commits.push(c),
        Err(_) => (),
    };

    let id = index.write_tree().unwrap();
    let tree = repo.find_tree(id).unwrap();

    repo.commit(
        Some("HEAD"),
        &repo.signature().unwrap(),
        &repo.signature().unwrap(),
        "commit message",
        &tree,
        &parent_commits,
    )
    .unwrap();
}
