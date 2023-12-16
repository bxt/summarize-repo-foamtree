use std::vec;

use git2::{Blob, Commit, ObjectType, Repository, Signature, Tag, Tree, TreeWalkMode, TreeWalkResult};

fn run(repo_path: &str) -> Result<(), git2::Error> {
    // let path = repo_path.as_ref().map(|s| &s[..]).unwrap_or(".");
    let repo = Repository::open(repo_path)?;
    let rev_options = ["origin/main", "origin/master", "main"];

    let err_none_found = Err(git2::Error::from_str("None of the common revs found"));
    let rev = rev_options.into_iter().fold(err_none_found, |acc, rev_option| {
        acc.or_else(|_|{
            let rev = repo.revparse_single(rev_option)?;
            let kind = rev.kind().ok_or(git2::Error::from_str(&format!("Rev {rev_option} has no kind?")))?;
            if kind == ObjectType::Commit {
                Ok(rev)
            } else {
                Err(git2::Error::from_str(&format!("Rev {rev_option} not a commit but a {:?}", kind)))
            }
        })
    })?;

    let commit = rev.as_commit().unwrap();

    let tree = commit.tree()?;

    let mut count = 0;
    let mut roots: Vec<String> = vec!["".to_string()];
    let mut root_weights: Vec<u64> = vec![0];

    println!("document.title = \"{repo_path}\";");
    println!("var dataObject = {{");
    println!("  groups: [");

    tree.walk(TreeWalkMode::PreOrder, |root,entry| {
        let kind = entry.kind().expect("Tree node without kind?");
        let name = entry.name().expect("Tree node without name?");

        while roots.join("") != root {
            let ident = roots.len()*4+4;
            let weight = root_weights.pop().unwrap();
            *root_weights.last_mut().unwrap() += weight;
            println!("{:ident$}], weight: {weight} }},","");
            roots.pop();
        }
        let ident = roots.len()*4+4;

        if kind == ObjectType::Tree {
            println!("{:ident$}{{ label: \"{name}\", groups: [","");
            // println!("> {root}{name}");
            roots.push(name.to_string() + "/");
            root_weights.push(1);
        } else if kind == ObjectType::Blob {
            println!("{:ident$}{{ label: \"{name}\", weight: 1 }},","");
            // println!("- {root}{name}");
            *root_weights.last_mut().unwrap() += 1;
        } else {
            panic!("Encountered object {} of kind {}",entry.id(),entry.kind().unwrap().str());
        }

        // dbg!(a, entry.name().unwrap(), entry.);
        count += 1;
        if count > 500000000 {
            TreeWalkResult::Abort
        } else {
            TreeWalkResult::Ok
        }
    })?;

    println!("  ],");
    println!("}};");

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let repo_path = if args.len() < 2 { "."} else { &args[1] };

    let start = std::time::Instant::now();

    match run(repo_path) {
        Ok(()) => {eprintln!("Done! 🎉")}
        Err(e) => eprintln!("Error: {}", e),
    }

    eprintln!("Took: {:.2?}", start.elapsed());
}