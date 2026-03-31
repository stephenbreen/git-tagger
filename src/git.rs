use chrono::{DateTime, Utc, TimeZone};
use color_eyre::Result;
use git2::{Repository, Oid};

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub name: String,
    pub commit_id: Oid,
    pub message: Option<String>,
    pub author: Option<String>,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub id: Oid,
    pub message: String,
    pub author: String,
}

pub fn list_tags(repo: &Repository) -> Result<Vec<TagInfo>> {
    let mut tags = Vec::new();
    let tag_names = repo.tag_names(None)?;

    for name in tag_names.iter().flatten() {
        let obj = repo.revparse_single(name)?;
        
        let (commit_id, message, author, date) = if let Some(tag) = obj.as_tag() {
            // Annotated tag
            let commit = tag.target()?.peel_to_commit()?;
            let time = commit.time();
            let date = Utc.timestamp_opt(time.seconds(), 0).unwrap();
            (
                commit.id(),
                tag.message().map(|s: &str| s.to_string()),
                tag.tagger().map(|t: git2::Signature| t.name().unwrap_or("").to_string()),
                date,
            )
        } else if let Some(commit) = obj.as_commit() {
            // Lightweight tag
            let time = commit.time();
            let date = Utc.timestamp_opt(time.seconds(), 0).unwrap();
            (
                commit.id(),
                None,
                None,
                date,
            )
        } else {
            continue;
        };

        tags.push(TagInfo {
            name: name.to_string(),
            commit_id,
            message,
            author,
            date,
        });
    }

    Ok(tags)
}

pub fn get_commits_between(
    repo: &Repository,
    from: Oid,
    to: Oid,
) -> Result<Vec<CommitInfo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push(to)?;
    revwalk.hide(from).ok(); // ok to fail if from is not reachable from to

    let mut commits = Vec::new();
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        commits.push(CommitInfo {
            id: oid,
            message: commit.message().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("").to_string(),
        });
    }

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use git2::{Repository, Signature};

    fn setup_repo() -> (TempDir, Repository) {
        let td = TempDir::new().unwrap();
        let repo = Repository::init(td.path()).unwrap();
        (td, repo)
    }

    #[test]
    fn test_list_tags_and_commits() {
        let (_td, repo) = setup_repo();
        let sig = Signature::now("Test User", "test@example.com").unwrap();
        
        // Initial commit
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let c1_id = repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[]).unwrap();
        repo.tag_lightweight("v0.1.0", &repo.find_object(c1_id, None).unwrap(), false).unwrap();

        // Second commit
        let c1 = repo.find_commit(c1_id).unwrap();
        let c2_id = repo.commit(Some("HEAD"), &sig, &sig, "second", &tree, &[&c1]).unwrap();
        repo.tag("v0.2.0", &repo.find_object(c2_id, None).unwrap(), &sig, "annotated tag", false).unwrap();

        let tags = list_tags(&repo).unwrap();
        assert_eq!(tags.len(), 2);
        
        let v01 = tags.iter().find(|t| t.name == "v0.1.0").unwrap();
        let v02 = tags.iter().find(|t| t.name == "v0.2.0").unwrap();
        
        assert!(v01.message.is_none());
        assert!(v02.message.is_some());

        let commits = get_commits_between(&repo, c1_id, c2_id).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].message, "second");
    }
}
