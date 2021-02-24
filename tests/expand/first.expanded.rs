fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    fn inner_main() -> std::result::Result<i32, Box<dyn std::error::Error>> {
        {
            Ok(0)
        }
    }
    std::process::exit(inner_main()?);
}
