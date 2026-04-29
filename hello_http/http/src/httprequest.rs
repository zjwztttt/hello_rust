use std::collections::HashMap;

///枚举请求方法
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

///为请求方法实现From特征
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

///枚举HTTP版本
#[derive(Debug, PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}

///为HTTP版本也实现From特征
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

///枚举资源路径
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

///定义HTTP请求结构
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

///为HTTP结构实现From特征
impl From<String> for HttpRequest {
    fn from(s: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in s.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {

            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest{
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

///处理请求行的方法
fn process_req_line(s :&str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

/// 处理请求头的方法
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

///测试代码
#[cfg(test)]
mod tests {
    use super::*;

    ///测试请求方法
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    ///测试HTTP版本
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    ///测试http方法
    #[test]
    fn test_read_http() {
        let s:String = String::from("GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.71.1\r\nAccept:*/*\r\n\r\n");
        let mut headers_expexted = HashMap::new();
        headers_expexted.insert("Host".into(), "localhost".into());
        headers_expexted.insert("Accept".into(), "*/*".into());
        headers_expexted.insert("User-Agent".into(), "curl/7.71.1".into());

        let req: HttpRequest = s.into();

        assert_eq!(Method::Get,req.method);
        assert_eq!(Version::V1_1,req.version);
        assert_eq!(Resource::Path("/greeting".to_string()),req.resource);
        assert_eq!(headers_expexted,req.headers);
    }

}