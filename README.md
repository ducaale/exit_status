# exit_status

Simple proc-macro for enabling rust main function to return an exit status code.

## Example
```rust
#[exit_status::main]
fn main() -> anyhow::Result<i32> {
    Ok(0)
}

// expands to
// fn main() -> anyhow::Result<()> {
//     fn inner_main() -> anyhow::Result<i32> {
//         {
//             Ok(0)
//         }
//     }
//     std::process::exit(inner_main()?);
// }
```