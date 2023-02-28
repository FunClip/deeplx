# deeplx

Free DeepL API written in Rust

## Description

`deeplx` run in port `14869`.

### Request

```http
POST /translate HTTP/1.1
Host: 127.0.0.1:14869
Content-Type: application/json
Content-Length: 79

{
    "text": "text to be translated",
    "source_lang": "auto",
    "target_lang": "ZH"
}
```

### Response

Content-Type: application/json
```json
{
    "code": 200,
    "msg": "error messages",
    "data": "translate result"
}
```

## Reference

[DeepLX](https://github.com/OwO-Network/DeepLX): Permanently free DeepL API written in Golang

[zu1k/deepl](https://hub.docker.com/r/zu1k/deepl): DeepL Free API Docker

[deepl-free-translate-service](https://github.com/akl7777777/deepl-free-translate-service): token free unlimited use of deepl, according to the web version of JavaScript encryption algorithm developed services; So as long as the official website algorithm should not, theoretically can be unlimited use
