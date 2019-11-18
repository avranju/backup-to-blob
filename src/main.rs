use std::fs::{self, DirEntry /*File*/};
use std::io::{self /*Write*/};
use std::path::{Path, PathBuf};

use async_std::task;
use futures::future::TryFutureExt;
use structopt::StructOpt;
//use tar::Builder;
//use xz2::write::XzEncoder;

mod azure;
mod blob_client;

use crate::azure::Login as AzureLogin;

const APP_TENANT_ID: &'static str = "0fff69a3-7670-4b49-b774-8d6921a30cb1";
const APP_CLIENT_ID: &'static str = "418b7705-aa1f-4845-abc8-4dbc07630cd2";

#[derive(Debug, StructOpt)]
pub struct AzureStorageOpt {
    #[structopt(short, long)]
    account: String,

    #[structopt(short, long)]
    container: String,
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    source_folder: PathBuf,

    #[structopt(flatten)]
    storage: AzureStorageOpt,

    #[structopt(short, long)]
    file_name: String,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    // let file = File::create(&opt.file_name)?;
    //    let marker = blob_client::BlobClient::new(&opt.storage, &opt.file_name);
    //    let mut tar_archive = Builder::new(XzEncoder::new(marker, 6));
    //    // let mut tar_archive = Builder::new(marker);
    //
    //    visit_dirs(&opt.source_folder, &mut |e| {
    //        let path = e.path();
    //        let name = path
    //            .strip_prefix(&opt.source_folder)
    //            .expect("prefix removal failed");
    //        let mut file = File::open(e.path())?;
    //
    //        println!("Adding {}", name.to_str().unwrap());
    //        tar_archive.append_file(name, &mut file)?;
    //        Ok(())
    //    })?;
    //
    //    let mut client = tar_archive.into_inner()?.finish()?;
    //    // let mut client = tar_archive.into_inner()?;
    //    client.flush()?;
    //    println!("All done.");

    let login = AzureLogin::new(APP_TENANT_ID.to_owned(), APP_CLIENT_ID.to_owned());
    task::block_on(async {
        let device_code = login
            .get_device_code()
            .map_err(|err| err.to_string())
            .await?;
        println!("{:?}", device_code);
        Ok(())
    })
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
