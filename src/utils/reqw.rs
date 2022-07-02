


use reqwest::{ Client,multipart};

use super::httpclient::HttpBinResult;


async fn reqwest_multipart_form(url:&str) -> anyhow::Result<HttpBinResult> {

    let client = Client::new();



    let bio = multipart::Part::text("hallo peeps")
    .file_name("bio.txt")
    .mime_str("text/plain")?;

    let form = multipart::Form::new()
    .text("username", "seanmonstar")
    .text("password", "secret")
    .part("file", bio);


    let response = client.post(url).multipart(form).send().await?;
    //let t = response.text().await?;
    //println!("{:?}",t   );
    let result = response.json::<HttpBinResult>().await?;

    Ok(result)
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
