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

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("text_to_binary", text_to_binary)?;
    cx.export_function("binary_to_text", binary_to_text)?;

    Ok(())
}
