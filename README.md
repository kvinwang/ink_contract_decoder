# ink contract decoder

A Rust crate to decode ink! contracts JSON into Rust types using Serde.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ink_contract_decoder = "0.1.0"
```

## Example

```
use ink_contract_decoder::decode_ink_contract;

fn main() {
    let json_str = r#"
    {
        // Your JSON contract string here
    }
    "#;

    let ink_contract = decode_ink_contract(json_str).unwrap();
    println!("{:#?}", ink_contract);
}
```
