use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

fn init_data(from: &str, to: &str, text: &str) -> Value {
    return json!({
        "header": {
            "fn": "auto_translation",
            "client_key": "browser-chrome-110.0.0-Mac OS-df4bd4c5-a65d-44b2-a40f-42f34f3535f2-1677486696487"
        },
        "type": "plain",
        "model_category": "normal",
        "source": {
            "lang": from,
            "text_list": [
                text
            ]
        },
        "target": {
            "lang": to
        }
    });
}

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    const URL: &str = "https://yi.qq.com/api/imt";

    let post_data = init_data(from, to, text);

    let res:Value= client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
        .header("referer", "https://yi.qq.com/zh-CN/index")
        .body(post_data.to_string())
        .send()?
        .json()?;
    fn parse_result(res: Value) -> Option<String> {
        let mut result = String::new();
        for line in res.as_object()?.get("auto_translation")?.as_array()? {
            result.push_str(line.as_str()?);
            result.push_str("\n");
        }
        Some(result.trim().to_string())
    }
    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("你好 世界！", "auto", "en", needs).unwrap();
        println!("{result}");
    }
}
