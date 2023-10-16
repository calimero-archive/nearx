use std::borrow::Cow;

use crate::warn;

pub const TGAS: near_primitives::types::Gas = 1000000000000;
pub const NEAR: near_primitives::types::Balance = 1000000000000000000000000;

// todo! add parsers around this "5N", "5Tgas", "5YoctoNear", "5Near", "5Ⓝ"

pub fn print_result(result: Cow<[u8]>) {
    let mut result = result;
    if let Ok(utf8_result) = std::str::from_utf8(&result) {
        if let Ok(json) = utf8_result.parse::<serde_json::Value>() {
            match serde_json::from_value::<Vec<u8>>(json.clone()) {
                Ok(bytes) if !bytes.is_empty() => {
                    result = Cow::from(bytes);
                }
                _ => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&json).expect("json is valid")
                    );
                    return;
                }
            }
        } else {
            println!("{}", utf8_result);
            return;
        }
    }
    warn!("the result is not valid utf-8");
    for byte in result.as_ref() {
        print!("{:02x}", byte);
    }
    println!();
}
