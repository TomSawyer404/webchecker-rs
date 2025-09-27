use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// 配置结构体
#[derive(Debug, Deserialize, Clone)]
struct Config {
    #[serde(rename = "user-agent")]
    user_agent: Option<String>,
    cookie: Option<String>,
    timeout: Option<u64>,
    headers: Option<HashMap<String, String>>,
}

// 访问结果结构体
#[derive(Debug, Serialize)]
struct CheckResult {
    original_url: String,
    status_code: u16,
    title: String,
    banner: String,
    content_length: usize,
    redirect_url: String,
    error: Option<String>,
}

// 异步URL检查命令
#[tauri::command]
async fn check_url(url: String, config: Config) -> Result<CheckResult, String> {
    // 创建HTTP客户端
    let client_builder = reqwest::Client::builder();

    // 设置超时时间
    let timeout_secs = config.timeout.unwrap_or(30);
    let client_builder = client_builder.timeout(Duration::from_secs(timeout_secs));

    // 设置User-Agent
    let client_builder = if let Some(ua) = config.user_agent {
        client_builder.user_agent(ua)
    } else {
        client_builder.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
    };

    let client = match client_builder.build() {
        Ok(client) => client,
        Err(e) => {
            return Ok(CheckResult {
                original_url: url.clone(),
                status_code: 0,
                title: "客户端创建失败".to_string(),
                banner: "".to_string(),
                content_length: 0,
                redirect_url: "".to_string(),
                error: Some(format!("客户端创建失败: {}", e)),
            });
        }
    };

    // 构建请求
    let mut request_builder = client.get(&url);

    // 设置Cookie
    if let Some(cookie) = config.cookie {
        request_builder = request_builder.header("Cookie", cookie);
    }

    // 设置自定义头部
    if let Some(headers) = config.headers {
        for (key, value) in headers {
            request_builder = request_builder.header(&key, value);
        }
    }

    // 发送请求
    let response = match request_builder.send().await {
        Ok(resp) => resp,
        Err(e) => {
            return Ok(CheckResult {
                original_url: url.clone(),
                status_code: 0,
                title: "请求失败".to_string(),
                banner: "".to_string(),
                content_length: 0,
                redirect_url: "".to_string(),
                error: Some(format!("请求失败: {}", e)),
            });
        }
    };

    // 获取状态码
    let status_code = response.status().as_u16();

    // 获取重定向URL
    let redirect_url = response
        .headers()
        .get("location")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_string();

    // 提取服务器信息（banner） - 在读取响应体之前
    let banner = extract_banner(&response);

    // 获取响应内容
    let content = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            return Ok(CheckResult {
                original_url: url.clone(),
                status_code,
                title: "内容读取失败".to_string(),
                banner,
                content_length: 0,
                redirect_url: redirect_url.clone(),
                error: Some(format!("内容读取失败: {}", e)),
            });
        }
    };

    // 提取标题
    let title = extract_title(&content);

    Ok(CheckResult {
        original_url: url,
        status_code,
        title,
        banner,
        content_length: content.len(),
        redirect_url,
        error: None,
    })
}

// 提取HTML标题
fn extract_title(html: &str) -> String {
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html.find("</title>") {
            if start < end {
                return html[start + 7..end].trim().to_string();
            }
        }
    }
    "无标题".to_string()
}

// 提取服务器信息
fn extract_banner(response: &reqwest::Response) -> String {
    let mut banner_parts = Vec::new();

    // 服务器信息
    if let Some(server) = response.headers().get("server") {
        if let Ok(server_str) = server.to_str() {
            banner_parts.push(format!("Server: {}", server_str));
        }
    }

    // 其他有用的头部信息
    let interesting_headers = vec!["x-powered-by", "x-aspnet-version", "x-frame-options"];

    for header_name in interesting_headers {
        if let Some(header_value) = response.headers().get(header_name) {
            if let Ok(value_str) = header_value.to_str() {
                banner_parts.push(format!("{}: {}", header_name, value_str));
            }
        }
    }

    if banner_parts.is_empty() {
        "未知".to_string()
    } else {
        banner_parts.join(" | ")
    }
}

// 批量检查URL的命令
#[tauri::command]
async fn batch_check_urls(urls: Vec<String>, config: Config) -> Result<Vec<CheckResult>, String> {
    let mut results = Vec::new();
    let mut tasks = Vec::new();

    // 为每个URL创建异步任务
    for url in urls {
        let config_clone = Config {
            user_agent: config.user_agent.clone(),
            cookie: config.cookie.clone(),
            timeout: config.timeout,
            headers: config.headers.clone(),
        };

        let task = tokio::spawn(async move { check_url(url, config_clone).await });
        tasks.push(task);
    }

    // 等待所有任务完成
    for task in tasks {
        match task.await {
            Ok(Ok(result)) => results.push(result),
            Ok(Err(e)) => {
                results.push(CheckResult {
                    original_url: "未知".to_string(),
                    status_code: 0,
                    title: "任务执行失败".to_string(),
                    banner: "".to_string(),
                    content_length: 0,
                    redirect_url: "".to_string(),
                    error: Some(e),
                });
            }
            Err(e) => {
                results.push(CheckResult {
                    original_url: "未知".to_string(),
                    status_code: 0,
                    title: "任务执行失败".to_string(),
                    banner: "".to_string(),
                    content_length: 0,
                    redirect_url: "".to_string(),
                    error: Some(format!("任务执行失败: {}", e)),
                });
            }
        }
    }

    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![check_url, batch_check_urls])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
