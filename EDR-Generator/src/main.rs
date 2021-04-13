use std::error::Error;
use crate::modules::process::ProcessManager;


mod modules;

fn main()-> Result<(), Box<dyn Error>> {
    println!("EDR-Generator") ;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
