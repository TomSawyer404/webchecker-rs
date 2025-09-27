<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 响应式数据
const targets = ref("");
const userAgent = ref("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");
const cookie = ref("");
const timeout = ref(30);
const headers = ref("");
const results = ref([]);
const isRunning = ref(false);

// 开始访问函数
async function startCheck() {
  if (!targets.value.trim()) {
    alert("请输入待访问的网址");
    return;
  }
  
  isRunning.value = true;
  results.value = [];
  
  try {
    // 构建配置对象
    const configObj = {
      "user-agent": userAgent.value,
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
    
    // 调用后端接口（这里需要根据实际的后端命令修改）
    for (const url of urlList) {
      try {
        const result = await invoke("check_url", { 
          url: url.trim(),
          config: configObj 
        });
        results.value.push(result);
      } catch (error) {
        results.value.push({
          originalUrl: url.trim(),
          statusCode: "ERROR",
          title: "访问失败",
          banner: "",
          contentLength: 0,
          redirectUrl: "",
          error: error.toString()
        });
      }
    }
  } catch (error) {
    console.error("访问出错:", error);
  } finally {
    isRunning.value = false;
  }
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
          placeholder="请输入待访问的网址，每行一个"
          class="targets-input"
          rows="6"
        ></textarea>
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
            ></textarea>
          </div>
        </div>
      </div>
      
      <button 
        @click="startCheck" 
        :disabled="isRunning"
        class="start-button"
      >
        {{ isRunning ? '访问中...' : '开始访问' }}
      </button>
    </div>
    
    <!-- 右侧栏 -->
    <div class="right-content">
      <h3>访问结果</h3>
      <div class="results-container">
        <div v-if="results.length === 0" class="no-results">
          暂无访问结果，点击"开始访问"按钮开始检查
        </div>
        <div v-else class="results-list">
          <div 
            v-for="(result, index) in results" 
            :key="index" 
            class="result-item"
            :class="{ error: result.statusCode === 'ERROR' }"
          >
            <div class="result-header">
              <span class="url">{{ result.originalUrl }}</span>
              <span class="status" :class="{
                success: result.statusCode === 200,
                warning: result.statusCode >= 300 && result.statusCode < 400,
                error: result.statusCode >= 400 || result.statusCode === 'ERROR'
              }">
                {{ result.statusCode }}
              </span>
            </div>
            <div class="result-details">
              <div><strong>标题:</strong> {{ result.title || '无' }}</div>
              <div><strong>Banner:</strong> {{ result.banner || '无' }}</div>
              <div><strong>内容长度:</strong> {{ result.contentLength || 0 }}</div>
              <div v-if="result.redirectUrl">
                <strong>重定向:</strong> {{ result.redirectUrl }}
              </div>
              <div v-if="result.error" class="error-message">
                <strong>错误:</strong> {{ result.error }}
              </div>
            </div>
          </div>
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
  margin-top: 10px;
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

/* 右侧栏样式 */
.right-content {
  flex: 1;
  padding: 20px;
  background-color: white;
  overflow-y: auto;
}

.right-content h3 {
  margin: 0 0 20px 0;
  color: #495057;
  font-size: 18px;
  border-bottom: 2px solid #007bff;
  padding-bottom: 8px;
}

.results-container {
  height: calc(100vh - 80px);
}

.no-results {
  text-align: center;
  color: #6c757d;
  font-style: italic;
  margin-top: 50px;
  font-size: 16px;
}

.results-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.result-item {
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 15px;
  background-color: #f8f9fa;
  transition: all 0.2s;
}

.result-item:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.result-item.error {
  border-color: #dc3545;
  background-color: #f8d7da;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
  gap: 10px;
}

.url {
  font-weight: bold;
  color: #495057;
  word-break: break-all;
  flex: 1;
}

.status {
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: bold;
  white-space: nowrap;
}

.status.success {
  background-color: #d4edda;
  color: #155724;
}

.status.warning {
  background-color: #fff3cd;
  color: #856404;
}

.status.error {
  background-color: #f8d7da;
  color: #721c24;
}

.result-details {
  font-size: 14px;
  color: #6c757d;
  line-height: 1.5;
}

.result-details div {
  margin-bottom: 6px;
  display: flex;
  align-items: flex-start;
}

.result-details strong {
  min-width: 80px;
  color: #495057;
}

.error-message {
  color: #dc3545;
  font-weight: bold;
  background-color: rgba(220, 53, 69, 0.1);
  padding: 5px 8px;
  border-radius: 4px;
  margin-top: 5px;
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

  .config-item label {
    color: #f8f9fa;
  }

  .right-content {
    background-color: #212529;
  }

  .right-content h3 {
    color: #f8f9fa;
    border-bottom-color: #007bff;
  }

  .result-item {
    background-color: #343a40;
    border-color: #495057;
    color: #f8f9fa;
  }

  .result-item.error {
    background-color: #721c24;
    border-color: #dc3545;
  }

  .url {
    color: #f8f9fa;
  }

  .result-details {
    color: #adb5bd;
  }

  .result-details strong {
    color: #f8f9fa;
  }
}
</style>