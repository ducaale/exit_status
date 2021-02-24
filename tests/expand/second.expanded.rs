fn main() -> anyhow::Result<()> {
    fn inner_main() -> anyhow::Result<i32> {
        {
            Ok(0)
        }
    }
    std::process::exit(inner_main()?);
}
