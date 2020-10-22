use git2::Commit;
use git2::Index;
use git2::ObjectType;
use git2::Repository;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = &args[1];
    let file_name = &args[2];
    let repo = if Path::new(root).exists() {
        println!("open");
        Repository::open(root).unwrap()
    } else {
        println!("init");
        Repository::init(root).unwrap()
    };

    let mut index = stage(&repo, root, file_name);
    commit(&repo, &mut index);
}
fn stage(repo: &Repository, root: &str, file_name: &str) -> Index {
    let mut index = repo.index().unwrap();

    File::create(Path::new(root).join(file_name)).unwrap();

    index.add_path(Path::new(file_name)).unwrap();
    index.write().unwrap();

    return index;
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn commit(repo: &Repository, index: &mut git2::Index) {
    let lc_result = find_last_commit(&repo);

    let mut parent_commits: Vec<&Commit> = vec![];

    match lc_result.as_ref() {
        Ok(c) => parent_commits.push(c),
        Err(_) => (),
    };

    index.write().unwrap();
    let id = index.write_tree().unwrap();
    index.write().unwrap();
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
