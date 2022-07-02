

use hyper::{body::Buf};
use serde::{Deserialize,Serialize};
use hyper::{Body, Method,Client,  Request};


#[derive(Debug, Serialize, Deserialize)]
pub struct HttpBinResult {
    pub args: Option<Args>,
    pub data: Option<String>,
    pub files: Option<Files>,
    pub form: Option<Form>,
    pub headers: Option<Headers>,
    pub json: Option<serde_json::Value>,
    pub origin: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    pub a: Option<String>,
    pub b: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub file: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub password: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Accept")]
    pub accept: Option<String>,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: Option<String>,
    #[serde(rename = "Accept-Language")]
    pub accept_language: Option<String>,
    #[serde(rename = "Dnt")]
    pub dnt: Option<String>,
    #[serde(rename = "Host")]
    pub host: Option<String>,
    #[serde(rename = "Origin")]
    pub origin: Option<String>,
    #[serde(rename = "Referer")]
    pub referer: Option<String>,
    #[serde(rename = "Sec-Ch-Ua")]
    pub sec_ch_ua: Option<String>,
    #[serde(rename = "Sec-Ch-Ua-Mobile")]
    pub sec_ch_ua_mobile: Option<String>,
    #[serde(rename = "Sec-Ch-Ua-Platform")]
    pub sec_ch_ua_platform: Option<String>,
    #[serde(rename = "Sec-Fetch-Dest")]
    pub sec_fetch_dest: Option<String>,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: Option<String>,
    #[serde(rename = "Sec-Fetch-Site")]
    pub sec_fetch_site: Option<String>,
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,
    #[serde(rename = "X-Amzn-Trace-Id")]
    pub x_amzn_trace_id: Option<String>,
}


// fn default_client()->hyper::Client{
//     Client::builder().pool_idle_timeout(Duration::from_sec(30)).build_http()
// }

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

async fn request_json_get(url: hyper::Uri) -> anyhow::Result<HttpBinResult> {
    let client = Client::new();

    // Fetch the url...
    let res = client.get(url).await?;

    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;

    // try to parse as json with serde_json
    let get_result:HttpBinResult = serde_json::from_reader(body.reader())?;

    Ok(get_result)
}

async fn request_json_post(uri:&str) -> anyhow::Result<HttpBinResult> {
    let client = Client::new();

    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let json_body = serde_json::to_vec(&address)?;

    let req = Request::builder().method(Method::POST).uri(uri)
    .header("X-Custom-Foo", "bar")
    .body(Body::from(json_body))?;



    // Fetch the url...
    let res = client.request(req).await?;

    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;

    // try to parse as json with serde_json
    let get_result:HttpBinResult = serde_json::from_reader(body.reader())?;

    Ok(get_result)
}







#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[tokio::test]
    async fn test_json_get() {
        let url = "http://httpbin.org/get?a=1&b=true".parse::<hyper::Uri>().unwrap();
        println!("{:?}",url );
        let get_json = request_json_get(url).await.unwrap();
        // print users
        println!("users: {:#?}", get_json);
    }

    #[tokio::test]
    async fn test_json_post() {
        let url = "http://httpbin.org/post?a=1&b=true";
        let get_json = request_json_post(url).await.unwrap();
        // print users
        println!("users: {:#?}", get_json);
    }


}