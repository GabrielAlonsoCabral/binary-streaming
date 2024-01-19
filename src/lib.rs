use neon::prelude::*;

pub fn text_to_binary(mut cx: FunctionContext) -> JsResult<JsString> {
    let input_str: String = cx.argument::<JsString>(0)?.value(&mut cx);

    let mut binary_string: String = String::new();

    for c in input_str.chars() {
        let binary_representation: String = format!("{:b}", c as u8);

        let padded_binary_representation: String = format!("{:08}", binary_representation);

        binary_string.push_str(&padded_binary_representation);
    }

    Ok(cx.string(binary_string))
}

pub fn binary_to_text(mut cx: FunctionContext) -> JsResult<JsString> {
    let input_str: String = cx.argument::<JsString>(0)?.value(&mut cx);

    if input_str.len() % 8 != 0 {
        return Ok(cx.string("Invalid binary string length"));
    }

    let mut text: String = String::new();

    for chunk in input_str.split_whitespace() {
        if let Ok(decimal_value) = u8::from_str_radix(chunk, 2) {
            text.push(char::from(decimal_value));
        } else {
            return Ok(cx.string("Error converting binary to text"));
        }
    }

    Ok(cx.string(text))
}

fn get_quotient(a: u32, b: u32) -> u32 {
    return a / b;
}

fn get_remainder(a: u32, b: u32) -> u32 {
    return a % b;
}

fn rust_decimal_to_binary(decimal_number: u32, mut binary_string: String) -> String {
    let remainder = get_remainder(decimal_number, 2);
    binary_string.push_str(&remainder.to_string());

    let quotient = get_quotient(decimal_number, 2);

    if quotient == 0 {
        return binary_string.chars().rev().collect(); // Reverse the string
    }

    rust_decimal_to_binary(quotient, binary_string)
}

pub fn decimal_to_binary(mut cx: FunctionContext) -> JsResult<JsString> {
    let input_decimal: u32 = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let mut binary_string: String = String::from("");

    binary_string = rust_decimal_to_binary(input_decimal, binary_string);

    Ok(cx.string(binary_string))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("text_to_binary", text_to_binary)?;
    cx.export_function("binary_to_text", binary_to_text)?;
    cx.export_function("decimal_to_binary", decimal_to_binary)?;
    Ok(())
}
