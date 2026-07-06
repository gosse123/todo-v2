mod libs;
use libs::read_input;

fn main()->std::io::Result<()>{
    
    read_input("data/file.json")?;

    Ok(())
}
