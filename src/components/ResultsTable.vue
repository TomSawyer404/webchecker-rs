<template>
  <div class="results-container">
    <div class="results-header">
      <h3>访问结果</h3>
      <div class="stats" v-if="results.length > 0">
        共 {{ results.length }} 个结果
        <span class="success-count">{{ successCount }} 成功</span>
        <span class="error-count">{{ errorCount }} 失败</span>
        <span v-if="isRunning" class="progress-indicator">进行中...</span>
        <span v-else-if="completed" class="completed-indicator">已完成</span>
      </div>
    </div>
    
    <div class="results-content">
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
              :class="getRowClass(result)"
            >
              <td class="original-input-cell">
                <span class="original-input-text">{{ result.original_url }}</span>  <!-- 修改：直接显示原始输入，不做处理 -->
                <div v-if="result.error" class="error-tooltip">
                  {{ result.error }}
                </div>
              </td>
              <td class="protocol-cell">
                <span class="protocol-badge" :class="getProtocolClass(result.original_url)">
                  {{ getProtocol(result.original_url) }}
                </span>
              </td>
              <td class="status-cell">
                <span class="status-badge" :class="getStatusClass(result)">
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
</template>

<script setup>
import { computed } from 'vue';
import { getProtocol } from '../utils/urlUtils.js';  // 修改：移除getOriginalInput导入

const props = defineProps({
  results: {
    type: Array,
    default: () => []
  },
  isRunning: {
    type: Boolean,
    default: false
  },
  completed: {
    type: Boolean,
    default: false
  }
});

const successCount = computed(() => 
  props.results.filter(r => r.status_code === 200).length
);

const errorCount = computed(() => 
  props.results.filter(r => r.status_code >= 400 || r.error).length
);

function getRowClass(result) {
  if (result.status_code === 200) return 'row-success';
  if (result.status_code >= 300 && result.status_code < 400) return 'row-redirect';
  if (result.status_code >= 400 || result.error) return 'row-error';
  return '';
}

function getProtocolClass(url) {
  const protocol = getProtocol(url);
  return `protocol-${protocol.toLowerCase()}`;
}

function getStatusClass(result) {
  if (result.status_code === 200) return 'status-success';
  if (result.status_code >= 300 && result.status_code < 400) return 'status-warning';
  if (result.status_code >= 400 || result.error) return 'status-error';
  return '';
}
</script>

<style scoped>
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

.results-content {
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