// URL处理工具函数
export function getProtocol(url) {
    if (url.startsWith('https://')) return 'HTTPS';
    if (url.startsWith('http://')) return 'HTTP';
    return '未知';
}

export function getOriginalInput(url) {
    if (url.startsWith('https://')) return url.substring(8);
    if (url.startsWith('http://')) return url.substring(7);
    return url;
}

export function parseUrlList(input) {
    return input.trim().split('\n').filter(url => url.trim());
}

export function parseHeaders(headersInput) {
    if (!headersInput.trim()) return {};

    const headerLines = headersInput.trim().split('\n');
    const customHeaders = {};
    headerLines.forEach(line => {
        const [key, value] = line.split(':').map(s => s.trim());
        if (key && value) {
            customHeaders[key] = value;
        }
    });
    return customHeaders;
}