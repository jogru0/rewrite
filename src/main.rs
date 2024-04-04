use std::env;

use anyhow::Error;
use git2::{Repository, Sort};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let repo = Repository::open(".")?;

    let parent = repo.find_branch(&args[1], git2::BranchType::Local)?;
    let parent = parent.get().peel(git2::ObjectType::Any)?.id();

    let branch = repo.head()?.peel(git2::ObjectType::Any)?.id();

    let Ok(merge_base) = repo.merge_base(parent, branch) else {
        assert!(repo.merge_base(branch, parent).is_err());

        //No common merge_base, so don't do anything for now.
        println!("No common merge base.");
        return Err(anyhow::Error::msg("No common merge base found."));
    };

    let merge_base_2 = repo.merge_base(branch, merge_base)?;
    assert_eq!(merge_base, merge_base_2);

    let mut revwalk = repo.revwalk()?;

    revwalk.push(branch)?;
    revwalk.hide(parent)?;

    revwalk.set_sorting(Sort::REVERSE)?;

    for rev in revwalk {
        let oid = rev?;

        let commit = repo.find_commit(oid)?;

        let message = commit.message().unwrap();

        let new_message = format!("COST | {message}");

        commit.amend(
            None,
            None,
            Some(&repo.signature()?),
            None,
            Some(&new_message),
            None,
        )?;
    }

    Ok(())
}
