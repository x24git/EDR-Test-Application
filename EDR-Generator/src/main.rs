use crate::modules::process::ProcessManager;
use crate::modules::common::GenerationError;
use crate::modules::file_system;
use std::thread;
use std::time::Duration;


mod modules;

fn main()-> Result<(), GenerationError> {

    let mut process_manager = ProcessManager::new()?;
    process_manager.new_process(String::from("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe"), Some(String::from("google.com msn.com")))?;
    thread::sleep(Duration::from_millis(4000));
    let result = process_manager.stop_all()?;
    println!("{:?}", result);
    println!("Process Success");

    file_system::new_file(String::from("test.txt"))?;
    file_system::mod_file(String::from("test.txt"))?;
    file_system::delete_file(String::from("test.txt"))?;
    println!("File Success");

    Ok(())





}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
