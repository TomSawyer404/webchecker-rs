<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// 响应式数据
const targets = ref("");
const userAgent = ref("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");
const cookie = ref("");
const timeout = ref(30);
const headers = ref("");
const results = ref([]);
const isRunning = ref(false);
const completed = ref(false);

// 事件监听器
let unlistenResult = null;
let unlistenComplete = null;

// 设置事件监听
async function setupEventListeners() {
  // 监听检查结果事件
  unlistenResult = await listen('check_result', (event) => {
    const result = event.payload;
    // 实时添加结果到列表
    results.value.push(result);
  });

  // 监听检查完成事件
  unlistenComplete = await listen('check_complete', () => {
    completed.value = true;
    isRunning.value = false;
  });
}

// 清理事件监听
function cleanupEventListeners() {
  if (unlistenResult) {
    unlistenResult();
    unlistenResult = null;
  }
  if (unlistenComplete) {
    unlistenComplete();
    unlistenComplete = null;
  }
}

onMounted(() => {
  setupEventListeners();
});

onUnmounted(() => {
  cleanupEventListeners();
});

// 开始访问函数
async function startCheck() {
  if (!targets.value.trim()) {
    alert("请输入待访问的网址");
    return;
  }
  
  // 重置状态
  isRunning.value = true;
  completed.value = false;
  results.value = [];
  
  try {
    // 构建配置对象
    const configObj = {
      "user_agent": userAgent.value,
      "cookie": cookie.value,
      "timeout": parseInt(timeout.value) || 30
    };
    
    // 解析自定义头部
    if (headers.value.trim()) {
      const headerLines = headers.value.trim().split('\n');
      const customHeaders = {};
      headerLines.forEach(line => {
        const [key, value] = line.split(':').map(s => s.trim());
        if (key && value) {
          customHeaders[key] = value;
        }
      });
      configObj.headers = customHeaders;
    }
    
    // 解析目标网址（按行分割）
    const urlList = targets.value.trim().split('\n').filter(url => url.trim());
    
    // 调用后端批量检查接口（现在返回空结果，通过事件接收实时结果）
    await invoke("batch_check_urls", { 
      urls: urlList,
      config: configObj 
    });
    
  } catch (error) {
    console.error("访问出错:", error);
    alert("访问过程中出现错误: " + error.toString());
    isRunning.value = false;
  }
}

// 获取协议类型
function getProtocol(url) {
  if (url.startsWith('https://')) return 'HTTPS';
  if (url.startsWith('http://')) return 'HTTP';
  return '未知';
}

// 获取原始输入（去除协议）
function getOriginalInput(url) {
  if (url.startsWith('https://')) return url.substring(8);
  if (url.startsWith('http://')) return url.substring(7);
  return url;
}

// 停止检查
function stopCheck() {
  isRunning.value = false;
  completed.value = true;
}
</script>

<template>
  <div class="app-container">
    <!-- 左侧栏 -->
    <div class="left-sidebar">
      <div class="input-section">
        <h3>目标网址</h3>
        <textarea 
          v-model="targets" 
          placeholder="请输入待访问的网址，每行一个（支持域名、URL、IP、IP:port等形式）"
          class="targets-input"
          rows="6"
          :disabled="isRunning"
        ></textarea>
        <div class="input-hint">
          提示：如果没有指定协议（http/https），将同时尝试两种协议
        </div>
      </div>
      
      <div class="input-section">
        <h3>配置信息</h3>
        <div class="config-form">
          <div class="config-item">
            <label for="user-agent">User-Agent:</label>
            <input 
              id="user-agent"
              v-model="userAgent" 
              type="text" 
              placeholder="User-Agent字符串"
              class="config-input"
              :disabled="isRunning"
            />
          </div>
          
          <div class="config-item">
            <label for="cookie">Cookie:</label>
            <textarea 
              id="cookie"
              v-model="cookie" 
              placeholder="Cookie信息"
              class="config-textarea"
              rows="2"
              :disabled="isRunning"
            ></textarea>
          </div>
          
          <div class="config-item">
            <label for="timeout">超时时间(秒):</label>
            <input 
              id="timeout"
              v-model.number="timeout" 
              type="number" 
              min="1" 
              max="300"
              class="config-input"
              :disabled="isRunning"
            />
          </div>
          
          <div class="config-item">
            <label for="headers">自定义头部:</label>
            <textarea 
              id="headers"
              v-model="headers" 
              placeholder="自定义HTTP头部，每行一个，格式：HeaderName: HeaderValue"
              class="config-textarea"
              rows="3"
              :disabled="isRunning"
            ></textarea>
          </div>
        </div>
      </div>
      
      <div class="button-group">
        <button 
          @click="startCheck" 
          :disabled="isRunning"
          class="start-button"
        >
          {{ isRunning ? '访问中...' : '开始访问' }}
        </button>
        <button 
          v-if="isRunning"
          @click="stopCheck"
          class="stop-button"
        >
          停止
        </button>
      </div>
    </div>
    
    <!-- 右侧栏 -->
    <div class="right-content">
      <div class="results-header">
        <h3>访问结果</h3>
        <div class="stats" v-if="results.length > 0">
          共 {{ results.length }} 个结果
          <span class="success-count">{{ results.filter(r => r.status_code === 200).length }} 成功</span>
          <span class="error-count">{{ results.filter(r => r.status_code >= 400 || r.error).length }} 失败</span>
          <span v-if="isRunning" class="progress-indicator">进行中...</span>
          <span v-else-if="completed" class="completed-indicator">已完成</span>
        </div>
      </div>
      <div class="results-container">
        <div v-if="results.length === 0 && !isRunning" class="no-results">
          暂无访问结果，点击"开始访问"按钮开始检查
        </div>
        <div v-else class="results-table-container">
          <table class="results-table">
            <thead>
              <tr>
                <th>原始输入</th>
                <th>协议</th>
                <th>Status Code</th>
                <th>Title</th>
                <th>Banner</th>
                <th>Content Length</th>
                <th>重定向URL</th>
              </tr>
            </thead>
            <tbody>
              <tr 
                v-for="(result, index) in results" 
                :key="index"
                :class="{
                  'row-success': result.status_code === 200,
                  'row-redirect': result.status_code >= 300 && result.status_code < 400,
                  'row-error': result.status_code >= 400 || result.error
                }"
              >
                <td class="original-input-cell">
                  <span class="original-input-text">{{ getOriginalInput(result.original_url) }}</span>
                  <div v-if="result.error" class="error-tooltip">
                    {{ result.error }}
                  </div>
                </td>
                <td class="protocol-cell">
                  <span class="protocol-badge" :class="{
                    'protocol-http': getProtocol(result.original_url) === 'HTTP',
                    'protocol-https': getProtocol(result.original_url) === 'HTTPS'
                  }">
                    {{ getProtocol(result.original_url) }}
                  </span>
                </td>
                <td class="status-cell">
                  <span class="status-badge" :class="{
                    'status-success': result.status_code === 200,
                    'status-warning': result.status_code >= 300 && result.status_code < 400,
                    'status-error': result.status_code >= 400 || result.error
                  }">
                    {{ result.status_code || (result.error ? 'ERROR' : '未知') }}
                  </span>
                </td>
                <td class="title-cell">{{ result.title || '无' }}</td>
                <td class="banner-cell">{{ result.banner || '无' }}</td>
                <td class="length-cell">{{ result.content_length || 0 }}</td>
                <td class="redirect-cell">{{ result.redirect_url || '无' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
}

/* 左侧栏样式 */
.left-sidebar {
  width: 450px;
  background-color: #f8f9fa;
  border-right: 1px solid #e9ecef;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  overflow-y: auto;
}

.input-section h3 {
  margin: 0 0 15px 0;
  color: #495057;
  font-size: 16px;
  border-bottom: 2px solid #007bff;
  padding-bottom: 5px;
}

.targets-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 14px;
  resize: vertical;
  font-family: 'Courier New', monospace;
}

.input-hint {
  font-size: 12px;
  color: #6c757d;
  margin-top: 5px;
  font-style: italic;
}

/* 配置表单样式 */
.config-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.config-item {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.config-item label {
  font-weight: 600;
  color: #495057;
  font-size: 14px;
}

.config-input {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 14px;
}

.config-textarea {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 14px;
  resize: vertical;
  font-family: 'Courier New', monospace;
}

.config-input:focus,
.config-textarea:focus,
.targets-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.button-group {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.start-button {
  padding: 12px 24px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  flex: 1;
}

.start-button:hover:not(:disabled) {
  background-color: #0056b3;
  transform: translateY(-1px);
}

.start-button:disabled {
  background-color: #6c757d;
  cursor: not-allowed;
  transform: none;
}

.stop-button {
  padding: 12px 24px;
  background-color: #dc3545;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  flex: 1;
}

.stop-button:hover {
  background-color: #c82333;
  transform: translateY(-1px);
}

/* 右侧栏样式 */
.right-content {
  flex: 1;
  padding: 20px;
  background-color: white;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  border-bottom: 2px solid #007bff;
  padding-bottom: 8px;
}

.results-header h3 {
  margin: 0;
  color: #495057;
  font-size: 18px;
}

.stats {
  display: flex;
  gap: 15px;
  font-size: 14px;
  color: #6c757d;
  align-items: center;
}

.success-count {
  color: #28a745;
  font-weight: 600;
}

.error-count {
  color: #dc3545;
  font-weight: 600;
}

.progress-indicator {
  color: #007bff;
  font-weight: 600;
  animation: pulse 1.5s infinite;
}

.completed-indicator {
  color: #28a745;
  font-weight: 600;
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.5; }
  100% { opacity: 1; }
}

.results-container {
  flex: 1;
  overflow-y: auto;
}

.no-results {
  text-align: center;
  color: #6c757d;
  font-style: italic;
  margin-top: 50px;
  font-size: 16px;
}

/* 表格样式 */
.results-table-container {
  overflow-x: auto;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  background-color: white;
}

.results-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
}

.results-table th {
  background-color: #f8f9fa;
  padding: 12px 15px;
  text-align: left;
  font-weight: 600;
  color: #495057;
  border-bottom: 2px solid #e9ecef;
  position: sticky;
  top: 0;
  z-index: 10;
}

.results-table td {
  padding: 12px 15px;
  border-bottom: 1px solid #e9ecef;
  vertical-align: top;
}

.results-table tbody tr:hover {
  background-color: #f8f9fa;
}

/* 行状态样式 */
.row-success {
  border-left: 4px solid #28a745;
}

.row-redirect {
  border-left: 4px solid #ffc107;
}

.row-error {
  border-left: 4px solid #dc3545;
  background-color: #f8d7da;
}

/* 单元格样式 */
.original-input-cell {
  max-width: 200px;
  position: relative;
}

.original-input-text {
  display: block;
  word-break: break-all;
  color: #495057;
  font-weight: 500;
}

.error-tooltip {
  font-size: 12px;
  color: #dc3545;
  background-color: rgba(220, 53, 69, 0.1);
  padding: 4px 8px;
  border-radius: 4px;
  margin-top: 4px;
  border-left: 2px solid #dc3545;
}

.protocol-cell {
  width: 80px;
}

.protocol-badge {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: bold;
  display: inline-block;
  min-width: 50px;
  text-align: center;
}

.protocol-http {
  background-color: #d1ecf1;
  color: #0c5460;
}

.protocol-https {
  background-color: #d4edda;
  color: #155724;
}

.status-cell {
  width: 100px;
}

.status-badge {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: bold;
  display: inline-block;
  min-width: 50px;
  text-align: center;
}

.status-success {
  background-color: #d4edda;
  color: #155724;
}

.status-warning {
  background-color: #fff3cd;
  color: #856404;
}

.status-error {
  background-color: #f8d7da;
  color: #721c24;
}

.title-cell {
  max-width: 200px;
  word-break: break-word;
}

.banner-cell {
  max-width: 150px;
  word-break: break-word;
}

.length-cell {
  width: 100px;
  text-align: right;
  font-family: 'Courier New', monospace;
}

.redirect-cell {
  max-width: 200px;
  word-break: break-all;
  color: #6c757d;
}

/* 响应式表格 */
@media (max-width: 1200px) {
  .results-table {
    font-size: 13px;
  }
  
  .results-table th,
  .results-table td {
    padding: 8px 10px;
  }
  
  .original-input-cell,
  .title-cell,
  .banner-cell,
  .redirect-cell {
    max-width: 150px;
  }
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  margin: 0;
  padding: 0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .left-sidebar {
    background-color: #343a40;
    border-right-color: #495057;
  }

  .input-section h3 {
    color: #f8f9fa;
    border-bottom-color: #007bff;
  }

  .targets-input,
  .config-input,
  .config-textarea {
    background-color: #495057;
    color: #f8f9fa;
    border-color: #6c757d;
  }

  .input-hint {
    color: #adb5bd;
  }

  .config-item label {
    color: #f8f9fa;
  }

  .right-content {
    background-color: #212529;
  }

  .results-header h3 {
    color: #f8f9fa;
  }

  .stats {
    color: #adb5bd;
  }

  .results-table-container {
    border-color: #495057;
    background-color: #343a40;
  }

  .results-table th {
    background-color: #495057;
    color: #f8f9fa;
    border-bottom-color: #6c757d;
  }

  .results-table td {
    border-bottom-color: #495057;
    color: #f8f9fa;
  }

  .results-table tbody tr:hover {
    background-color: #495057;
  }

  .row-success {
    border-left-color: #28a745;
  }

  .row-redirect {
    border-left-color: #ffc107;
  }

  .row-error {
    border-left-color: #dc3545;
    background-color: #721c24;
  }

  .original-input-text {
    color: #f8f9fa;
  }

  .error-tooltip {
    color: #f8d7da;
    background-color: rgba(220, 53, 69, 0.2);
    border-left-color: #dc3545;
  }

  .protocol-http {
    background-color: #0c5460;
    color: #d1ecf1;
  }

  .protocol-https {
    background-color: #155724;
    color: #d4edda;
  }

  .status-success {
    background-color: #155724;
    color: #d4edda;
  }

  .status-warning {
    background-color: #856404;
    color: #fff3cd;
  }

  .status-error {
    background-color: #721c24;
    color: #f8d7da;
  }

  .redirect-cell {
    color: #adb5bd;
  }
}
</style>
