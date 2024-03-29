use std::{collections::HashMap, path::Path, vec};

use git2::{BlameOptions, ObjectType, Repository, TreeWalkMode, TreeWalkResult};

fn run(repo_path: &str, do_blame: bool) -> Result<(), git2::Error> {
    // let path = repo_path.as_ref().map(|s| &s[..]).unwrap_or(".");
    let repo = Repository::open(repo_path)?;
    let rev_options = ["origin/main", "origin/master", "main"];

    let err_none_found = Err(git2::Error::from_str("None of the common revs found"));
    let rev = rev_options
        .into_iter()
        .fold(err_none_found, |acc, rev_option| {
            acc.or_else(|_| {
                let rev = repo.revparse_single(rev_option)?;
                let kind = rev.kind().ok_or(git2::Error::from_str(&format!(
                    "Rev {rev_option} has no kind?"
                )))?;
                if kind == ObjectType::Commit {
                    Ok(rev_option)
                } else {
                    Err(git2::Error::from_str(&format!(
                        "Rev {rev_option} not a commit but a {:?}",
                        kind
                    )))
                }
            })
        })?;

    let parsed_rev = repo.revparse_single(rev).unwrap();
    let commit = parsed_rev.as_commit().unwrap();

    let mut blame_options = BlameOptions::default();
    let mut blame_cutoff = None;

    if do_blame {
        let blame_start = repo
            .revparse_single(&(rev.to_string() + "~1000"))
            .expect("Blame reference commit not found")
            .as_commit()
            .expect("Blame reference commit not a commit?")
            .id();
        blame_cutoff = Some(blame_start);
        blame_options.oldest_commit(blame_start);
    }

    let tree = commit.tree()?;

    let mut count = 0;
    let mut roots: Vec<String> = vec!["".to_string()];
    let mut root_weights: Vec<usize> = vec![0];

    println!("document.title = \"{repo_path}\";");
    println!("var dataObject = {{");
    println!("  groups: [");

    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let kind = entry.kind().expect("Tree node without kind?");
        let name = entry.name().expect("Tree node without name?");

        while roots.join("") != root {
            let ident = roots.len() * 4 + 4;
            let weight = root_weights.pop().unwrap();
            *root_weights.last_mut().unwrap() += weight;
            let name = roots.pop();
            println!("{:ident$}], weight: {weight} }}, // end {name:?}", "");
        }
        let path = roots.join("");

        if path.starts_with("packages/common/graphql-types") {
            return TreeWalkResult::Skip;
        }

        let ident = roots.len() * 4 + 4;

        if kind == ObjectType::Tree {
            println!("{:ident$}{{ label: \"{name}\", groups: [", "");
            // println!("> {root}{name}");
            roots.push(name.to_string() + "/");
            root_weights.push(1);
        } else if kind == ObjectType::Blob {
            if name.ends_with(".snap")
                || name == "schema.graphql"
                || name == "yarn.lock"
                || name == "de.js"
                || name == "en.js"
                || name == "fr.js"
                || name == "es.js"
                || name == "it.js"
                || name.ends_with(".json")
                || name.ends_with(".jpg")
                || name.ends_with(".jpeg")
                || name.ends_with(".png")
                || name.ends_with(".webp")
                || name.ends_with(".pdf")
                || name.ends_with(".mp4")
                || name.ends_with(".woff")
                || name.ends_with(".svg")
                || name.ends_with(".ttf")
                || name.ends_with(".tar.gz")
                || name.ends_with(".gql-types.tsx")
            {
                return TreeWalkResult::Skip;
            }

            println!("{:ident$}{{ label: \"{name}\", groups: [", "");
            // println!("- {root}{name}");

            let size = if do_blame {
                let blame = repo.blame_file(Path::new(&(path + name)), Some(&mut blame_options)).unwrap();

                let mut people = HashMap::new();

                for blame_hunk in blame.iter() {
                    let size = blame_hunk.lines_in_hunk();
                    let is_cutoff = blame_cutoff.is_some_and(|it| it == blame_hunk.final_commit_id());
                    let author = if is_cutoff {
                        "Mr. Nobody".to_string()
                    } else {
                        let signature = blame_hunk.final_signature();
                        signature.name().unwrap_or("Mr. Nobody").to_string()
                    };
                    *people.entry(author).or_insert(0) += size;
                }

                let mut size = 0;
                for (index, (author, count)) in people.iter().enumerate() {
                    size += count;
                    println!("{:ident$}    {{ label: \"{name} hunk {index}\", weight: {count}, person: \"{author}\" }},", "");
                }

                size
            } else {
                entry.to_object(&repo).unwrap().as_blob().unwrap().size()
            };

            println!("{:ident$}], weight: {size} }}, // end {name}", "");
            *root_weights.last_mut().unwrap() += size;
        } else {
            panic!(
                "Encountered object {} of kind {}",
                entry.id(),
                entry.kind().unwrap().str()
            );
        }

        count += 1;
        if count % 1000 == 0 {
            eprintln!("Now {count} files processed.");
        }
        if count > 500000000 {
            TreeWalkResult::Abort
        } else {
            TreeWalkResult::Ok
        }
    })?;

    while roots.join("") != "" {
        let weight = root_weights.pop().unwrap();
        *root_weights.last_mut().unwrap() += weight;
        let name = roots.pop();
        let ident = roots.len() * 4 + 4;
        println!("{:ident$}], weight: {weight} }}, // end {name:?}", "");
    }

    println!("  ],");
    println!("}};");

    eprintln!("Now {count} files processed.");

    Ok(())
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    let mut do_blame = false;
    args.retain(|a| {
        (a == "--blame")
            .then(|| {
                do_blame = true;
            })
            .is_none()
    });

    let repo_path = if args.len() < 2 { "." } else { &args[1] };

    let start = std::time::Instant::now();

    match run(repo_path, do_blame) {
        Ok(()) => {
            eprintln!("Done! 🎉")
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    eprintln!("Took: {:.2?}", start.elapsed());
}
