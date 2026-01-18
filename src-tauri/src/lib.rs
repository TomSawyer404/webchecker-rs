use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tauri::Emitter;

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
#[derive(Debug, Serialize, Clone)]
struct CheckResult {
    original_url: String,
    original_input: String,
    status_code: u16,
    title: String,
    banner: String,
    content_length: usize,
    redirect_url: String,
    error: Option<String>,
}

// 解析和标准化URL
fn normalize_urls(input_url: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let url = input_url.trim();

    if url.is_empty() {
        return urls;
    }

    // 检查是否已经包含协议
    if url.starts_with("http://") || url.starts_with("https://") {
        urls.push(url.to_string());
    } else {
        // 如果没有协议，同时尝试http和https
        urls.push(format!("http://{}", url));
        urls.push(format!("https://{}", url));
    }

    urls
}

// 异步URL检查命令
#[tauri::command]
// 修改函数签名，使用引用
async fn check_url(
    url: String,
    original_input: &str, // 改为引用
    config: Config,
) -> Result<CheckResult, String> {
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
                original_input: original_input.to_string(),
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
                original_input: original_input.to_string(),
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
                original_input: original_input.to_string(),
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

    // 在所有使用original_input的地方都不需要.clone()
    Ok(CheckResult {
        original_url: url,
        original_input: original_input.to_string(), // 只在最后转换为String
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

// 批量检查URL的命令 - 实时返回结果
#[tauri::command]
async fn batch_check_urls(
    app_handle: tauri::AppHandle,
    urls: Vec<String>,
    config: Config,
) -> Result<(), String> {
    let mut tasks = Vec::new();

    // 为每个URL创建异步任务
    for input_url in urls {
        // 标准化URL（处理协议）
        let normalized_urls = normalize_urls(&input_url);

        for url in normalized_urls {
            let app_handle_clone = app_handle.clone();
            let config_clone = Config {
                user_agent: config.user_agent.clone(),
                cookie: config.cookie.clone(),
                timeout: config.timeout,
                headers: config.headers.clone(),
            };

            // 克隆input_url以避免移动问题
            let original_input = input_url.clone();

            let task = tokio::spawn(async move {
                let result = check_url(url.clone(), &original_input, config_clone).await;

                // 发送结果到前端
                match result {
                    Ok(check_result) => {
                        let _ = app_handle_clone.emit("check_result", check_result);
                    }
                    Err(e) => {
                        let error_result = CheckResult {
                            original_url: url,
                            original_input: original_input,
                            status_code: 0,
                            title: "检查失败".to_string(),
                            banner: "".to_string(),
                            content_length: 0,
                            redirect_url: "".to_string(),
                            error: Some(e),
                        };
                        let _ = app_handle_clone.emit("check_result", error_result);
                    }
                }
            });
            tasks.push(task);
        }
    }

    // 等待所有任务完成
    for task in tasks {
        let _ = task.await;
    }

    // 发送完成事件
    let _ = app_handle.emit("check_complete", "所有检查已完成");

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![check_url, batch_check_urls])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
