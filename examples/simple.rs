fn main() -> Result<(), Box<dyn std::error::Error>> {
    let os_information = os_type::current_platform();    
    println!("{:?}", os_information);
    Ok(())
}
