pub enum Object {
    Tree,
    Blob,
    Commit,
    Tag,
}

pub struct Blob {
    pub data: Vec<u8>,
}

pub struct Commit {
    /// The hash of recorded working tree state.
    pub tree: String,
    /// Hash of each parent commit. Empty for the first commit in repository.
    pub parents: Vec<Commit>,
    /// Who wrote this commit.
    pub author: String,
    /// Who committed this commit.
    ///
    /// This may be different from the `author` in case the author couldn't write to the repository themselves and
    /// is commonly encountered with contributed commits.
    pub committer: String,
    /// The name of the message encoding, otherwise [UTF-8 should be assumed](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/commit.c#L1493:L1493).
    pub encoding: Option<String>,
    /// The commit message documenting the change.
    pub message: String,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
