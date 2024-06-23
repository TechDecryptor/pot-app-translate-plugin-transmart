async function translate(text, from, to, options) {
    const { utils } = options;
    const { tauriFetch: fetch } = utils;

    const URL = "https://transmart.qq.com/api/imt";

    const body = {
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
    };
    const headers = {
        "Content-Type": "application/json",
        "user-agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36",
        "referer": "https://transmart.qq.com/zh-CN/index"
    };

    let res = await fetch(URL, {
        method: 'POST',
        headers: headers,
        body: {
            type: 'Json',
            payload: body
        },
    });

    if (res.ok) {
        let result = res.data;
        const { auto_translation } = result;
        if (auto_translation){
            return auto_translation.join("\n").trim();
        }else{
            throw JSON.stringify(result);
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}