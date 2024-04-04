use std::env;

use anyhow::Error;
use git2::Repository;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let repo = Repository::open(".")?;

    let parent_branch = repo.find_branch(&args[1], git2::BranchType::Local)?;
    let parent = parent_branch.get().peel(git2::ObjectType::Any)?.id();

    let branch = repo.head()?.peel(git2::ObjectType::Any)?.id();

    let Ok(merge_base) = repo.merge_base(parent, branch) else {
        assert!(repo.merge_base(branch, parent).is_err());

        //No common merge_base, so don't do anything for now.
        println!("No common merge base.");
        return Err(anyhow::Error::msg("No common merge base found."));
    };

    let merge_base_2 = repo.merge_base(branch, merge_base)?;
    assert_eq!(merge_base, merge_base_2);

    let parent = repo.reference_to_annotated_commit(parent_branch.get())?;

    let mut rebase = repo.rebase(None, Some(&parent), None, None)?;

    while let Some(maybe_op) = rebase.next() {
        let op = maybe_op?;
        dbg!(op.kind().unwrap());
        dbg!(op.id());

        let commit = repo.find_commit(op.id())?;

        let message = commit.message().unwrap();

        let new_message = format!("COST | {message}");

        rebase.commit(None, &repo.signature()?, Some(&new_message))?;
    }

    rebase.finish(None)?;

    Ok(())
}
