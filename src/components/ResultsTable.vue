<template>
  <div class="results-container">
    <div class="results-header">
      <h3>访问结果</h3>
      <div class="header-actions">
        <div class="stats" v-if="results.length > 0">
          共 {{ results.length }} 个结果
          <span class="success-count">{{ successCount }} 成功</span>
          <span class="error-count">{{ errorCount }} 失败</span>
          <span v-if="isRunning" class="progress-indicator">进行中...</span>
          <span v-else-if="completed" class="completed-indicator">已完成</span>
        </div>
        <button 
          v-if="results.length > 0 && !isRunning"
          @click="exportToXLSX"
          class="export-btn"
          title="导出为XLSX格式"
          :disabled="isExporting"
        >
          <span v-if="isExporting">🔄 导出中...</span>
          <span v-else>📊 导出XLSX</span>
        </button>



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
                <span class="original-input-text">{{ result.original_input }}</span>
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
import { computed, ref } from 'vue';
import { getProtocol } from '../utils/urlUtils.js';
import * as XLSX from 'xlsx';

const isExporting = ref(false);

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

function exportToXLSX() {
  console.log('exportToXLSX called');
  console.log('Results length:', props.results.length);
  console.log('XLSX library:', XLSX);
  
  if (props.results.length === 0) {
    console.log('No results to export');
    return;
  }

  isExporting.value = true;

  try {
    // 准备Excel数据
    const excelData = props.results.map(result => ({
      '原始输入': result.original_input || '',
      '协议': getProtocol(result.original_url) || '',
      'Status Code': result.status_code || (result.error ? 'ERROR' : ''),
      'Title': result.title || '',
      'Banner': result.banner || '',
      'Content Length': result.content_length || 0,
      '重定向URL': result.redirect_url || '',
      '错误信息': result.error || ''
    }));

    console.log('Excel data prepared:', excelData);

    // 创建工作簿
    const ws = XLSX.utils.json_to_sheet(excelData);
    console.log('Worksheet created:', ws);
    
    // 设置列宽
    const colWidths = [
      { wch: 30 }, // 原始输入
      { wch: 10 }, // 协议
      { wch: 15 }, // Status Code
      { wch: 40 }, // Title
      { wch: 30 }, // Banner
      { wch: 15 }, // Content Length
      { wch: 40 }, // 重定向URL
      { wch: 30 }  // 错误信息
    ];
    ws['!cols'] = colWidths;

    // 创建工作簿
    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, ws, '访问结果');
    console.log('Workbook created:', wb);

    // 生成文件名（包含时间戳）
    const now = new Date();
    const timestamp = now.toISOString().replace(/[:.]/g, '-').slice(0, -5);
    const filename = `webchecker-results-${timestamp}.xlsx`;
    console.log('Filename:', filename);

    // 保存文件
    XLSX.writeFile(wb, filename);
    console.log('File saved successfully');

    // 显示成功提示框
    alert(`✅ 导出成功！\n\n文件已保存为：${filename}\n\n文件已自动下载到您的默认下载文件夹中。\n\n您可以在浏览器的下载记录中查看文件位置。`);
    
  } catch (error) {
    console.error('Export error:', error);
    alert('❌ 导出失败: ' + error.message);
  } finally {
    isExporting.value = false;
  }
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

.header-actions {
  display: flex;
  align-items: center;
  gap: 20px;
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

.export-btn {
  background: linear-gradient(135deg, #28a745, #20c997);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 6px;
  box-shadow: 0 2px 4px rgba(40, 167, 69, 0.2);
}

.export-btn:hover {
  background: linear-gradient(135deg, #218838, #1e7e34);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(40, 167, 69, 0.3);
}

.export-btn:active {
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(40, 167, 69, 0.2);
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

@media (max-width: 768px) {
  .header-actions {
    flex-direction: column;
    gap: 10px;
  }
  
  .stats {
    flex-direction: column;
    gap: 5px;
    text-align: center;
  }
  
  .export-btn {
    width: 100%;
    justify-content: center;
  }
}

.export-btn:disabled {
  background: linear-gradient(135deg, #6c757d, #5a6268);
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.export-btn:disabled:hover {
  background: linear-gradient(135deg, #6c757d, #5a6268);
  transform: none;
  box-shadow: none;
}
</style>