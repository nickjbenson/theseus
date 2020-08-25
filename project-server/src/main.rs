use std::fs::DirEntry;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::path::Path;
use structopt::StructOpt;

// Constants
// ---------
//
const VERBOSE: bool = true;

// Command Line Interface
// ----------------------
//
#[derive(Debug, StructOpt)]
struct Cli {
  // The path of the project folder to open.
  #[structopt(parse(from_os_str))]
  project_path: std::path::PathBuf
} 

// Main
// ----
//
fn main() {
  // Project from path.
  let args = Cli::from_args();
  println!("{:?}", args);
  let project_path = Path::new(&args.project_path);
  let project_assets_path = project_path.join(Path::new("Assets"));
  if VERBOSE {
    println!("Project path: {:?}", args.project_path);
    println!("Project path exists? {:?}", project_path.exists());
    println!("Project assets path (theoretical): {:?}", project_assets_path);
    println!("Project assets path exists? {:?}", project_assets_path.exists());
  }
  if !project_assets_path.exists() {
    panic!("Project path \"{:?}\" is not a project because it does not contain an Assets folder.", project_path);
  }

  // Server life.
  let live = Arc::new(AtomicBool::from(true));
  
  // Ctrl+C handler.
  let live_for_ctrlc_handler = live.clone();
  ctrlc::set_handler(move || {
    println!("Received Ctrl+C.");
    live_for_ctrlc_handler.store(false, Ordering::SeqCst);
  })
  .expect("Error setting Ctrl+C handler.");
  
  println!("\nListing project directory contents...");
  print_dir_files(project_path);

  println!();
  println!("Theseus project server started.\nProject path: \"{:?}\"\nCtrl+C to stop the server.\n", project_assets_path.canonicalize().unwrap());
  loop {
    if live.load(Ordering::SeqCst) == false { break; }

    thread::sleep(Duration::from_millis(8));
  }
}

use std::io;
use std::fs;
// Recursively walk directories, call callback
// fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
fn print_dir_files(dir: &Path) -> io::Result<()> {
  if dir.is_dir() {
      for entry in fs::read_dir(dir)? {
          let entry = entry?;
          let path = entry.path();
          if path.is_dir() {
              // visit_dirs(&path, cb)?;
              println!("DIR: {:?}", entry);
              print_dir_files(&path)?;
          } else {
              // cb(&entry);
              println!("{:?}", entry);
          }
      }
  }
  Ok(())
}
