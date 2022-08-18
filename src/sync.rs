use futures::future;
use crate::{Args, file::File};

pub async fn sync_files(mut files: Vec<File>, args: &Args) -> Vec<File> {
    if args.sync {
        let repositories = files
            .iter_mut()
            .map(|f| f.get_description());

        future::try_join_all(repositories).await.expect("Error reading repository");
    }
    files
}
