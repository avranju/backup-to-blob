use std::fs::{self, DirEntry, File};
use std::io;
use std::path::{Path, PathBuf};

use structopt::StructOpt;
use tar::Builder;
use xz2::write::XzEncoder;

#[derive(Debug, StructOpt)]
struct AzureStorageOpt {
    #[structopt(short, long)]
    account: String,

    #[structopt(short, long)]
    key: String,

    #[structopt(short, long)]
    container: String,
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    source_folder: PathBuf,

    #[structopt(flatten)]
    storage: AzureStorageOpt,

    #[structopt(short, long)]
    file_name: String,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let file = File::create(&opt.file_name)?;
    let mut tar_archive = Builder::new(XzEncoder::new(file, 6));

    visit_dirs(&opt.source_folder, &mut |e| {
        let path = e.path();
        let name = path
            .strip_prefix(&opt.source_folder)
            .expect("prefix removal failed");
        let mut file = File::open(e.path())?;

        println!("Adding {}", name.to_str().unwrap());
        tar_archive.append_file(name, &mut file)?;
        Ok(())
    })?;

    tar_archive.into_inner()?.finish()?;
    Ok(())
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }

    Ok(())
}
