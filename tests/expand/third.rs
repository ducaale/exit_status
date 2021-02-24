
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[exit_status::main]
fn main() -> Result<i32> {
    Ok(0)
}