

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use reqwest::{Body, Client,multipart};
use super::httpclient::HttpBinResult;


async fn reqwest_multipart_form(url:&str) -> anyhow::Result<HttpBinResult> {

    let client = Client::new();




    //路径和 cargo.toml 在一个目录下面
    let file = File::open(".gitignore").await?;
    let file_body = file_to_body(file);

    let bio = multipart::Part::stream(file_body)
    .file_name("bio.txt")
    .mime_str("text/plain")?;

    let form = multipart::Form::new()
    .text("username", "seanmonstar")
    .text("password", "secret")
    .part("file", bio);


    let response = client.post(url).multipart(form).send().await?;
    // let print_string = response.text().await?;
    // println!("{:?}",print_string   );
    let result = response.json::<HttpBinResult>().await?;
    // let result = HttpBinResult { args: None, data:None, files:None, form:None, headers: None, json: None, origin: None, url: None };
    Ok(result)
}


fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[tokio::test]
    async fn test_post_form_file() {
        let url = "http://httpbin.org/post?a=1&b=true";
        let get_json = reqwest_multipart_form(url).await.unwrap();
        // print users
        println!("users: {:#?}", get_json);
    }

}

