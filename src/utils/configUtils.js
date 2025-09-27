// 需要导入parseHeaders函数
import { parseHeaders } from './urlUtils.js';

// 配置处理工具函数
export function buildConfig(userAgent, cookie, timeout, headers) {
    const configObj = {
        "user_agent": userAgent,
        "cookie": cookie,
        "timeout": parseInt(timeout) || 30
    };

    const customHeaders = parseHeaders(headers);
    if (Object.keys(customHeaders).length > 0) {
        configObj.headers = customHeaders;
    }

    return configObj;
}
