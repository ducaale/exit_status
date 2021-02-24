type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    fn inner_main() -> Result<i32> {
        {
            Ok(0)
        }
    }
    std::process::exit(inner_main()?);
}
