use notify::{Event, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use std::env;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub fn watch_feature_files(tx_main: std::sync::mpsc::Sender<Event>) -> NotifyResult<()> {
    let (tx, rx): (
        std::sync::mpsc::Sender<notify::Result<Event>>,
        Receiver<notify::Result<Event>>,
    ) = channel();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current working directory (file_watcher): {:?}", current_dir);

    let feature_files_dir = current_dir.parent().unwrap().join("feature-files");
    println!("Feature files directory (file_watcher): {:?}", feature_files_dir);

    let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default())?;
    watcher.watch(&feature_files_dir, RecursiveMode::Recursive)?;

    thread::spawn(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    tx_main.send(event).unwrap();
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    Ok(())
}
