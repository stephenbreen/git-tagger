use chrono::{DateTime, Utc, TimeZone};
use color_eyre::Result;
use git2::{Repository, Oid};

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub name: String,
    pub id: Oid,
    pub commit_id: Oid,
    pub message: Option<String>,
    pub author: Option<String>,
    pub date: DateTime<Utc>,
}

pub fn list_tags(repo: &Repository) -> Result<Vec<TagInfo>> {
    let mut tags = Vec::new();
    let tag_names = repo.tag_names(None)?;

    for name in tag_names.iter().flatten() {
        let obj = repo.revparse_single(name)?;
        
        let (commit_id, message, author, date) = if let Ok(tag) = obj.as_tag() {
            // Annotated tag
            let commit = tag.target()?.peel_to_commit()?;
            let time = commit.time();
            let date = Utc.timestamp_opt(time.seconds(), 0).unwrap();
            (
                commit.id(),
                tag.message().map(|s| s.to_string()),
                tag.tagger().map(|t| t.name().unwrap_or("").to_string()),
                date,
            )
        } else if let Ok(commit) = obj.as_commit() {
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
            id: obj.id(),
            commit_id,
            message,
            author,
            date,
        });
    }

    Ok(tags)
}
