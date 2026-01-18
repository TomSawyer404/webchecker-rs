<template>
  <div class="app-container">
    <!-- 菜单栏 -->
    <div class="menu-bar">
      <div class="menu-item" @click="toggleFileMenu">
        <span>文件</span>
        <div v-if="showFileMenu" class="dropdown-menu">
          <div class="dropdown-item" @click="importFile" :disabled="isImporting">
            <span v-if="isImporting">🔄 导入中...</span>
            <span v-else>📁 导入文件</span>
          </div>
          <div class="dropdown-item" @click="exportFile" :disabled="isExporting">
            <span v-if="isExporting">🔄 导出中...</span>
            <span v-else>💾 导出文件</span>
          </div>
        </div>
      </div>
      <div class="menu-item" @click="showAbout">
        <span>关于</span>
      </div>
      
      <!-- 状态信息 -->
      <div class="status-info" v-if="results.length > 0">
        <span class="stats">共 {{ results.length }} 个结果</span>
        <span class="success-count">{{ successCount }} 成功</span>
        <span class="error-count">{{ errorCount }} 失败</span>
        <span v-if="isRunning" class="progress-indicator">进行中...</span>
        <span v-else-if="completed" class="completed-indicator">已完成</span>
      </div>
      
      <!-- 操作按钮 -->
      <div class="action-buttons" v-if="results.length > 0 && !isRunning">
        <button 
          @click="clearHistory"
          class="clear-btn"
          title="清除所有历史记录"
        >
          🗑️ 清空历史
        </button>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="main-content">
      <!-- 左侧栏 -->
      <div class="left-sidebar">
        <UrlInput 
          v-model="targets"
          :disabled="isRunning"
        />
        
        <ConfigForm 
          :user-agent="userAgent"
          :cookie="cookie"
          :timeout="timeout"
          :headers="headers"
          :disabled="isRunning"
          @update:user-agent="userAgent = $event"
          @update:cookie="cookie = $event"
          @update:timeout="timeout = $event"
          @update:headers="headers = $event"
        />
        
        <ControlButtons 
          :is-running="isRunning"
          :disabled="isRunning"
          @start="startCheck"
          @stop="stopCheck"
        />
      </div>
      
      <!-- 右侧栏 -->
      <div class="right-content">
        <ResultsTable 
          :results="results"
          :is-running="isRunning"
          :completed="completed"
        />
      </div>
    </div>

    <!-- 关于对话框 -->
    <div v-if="showAboutDialog" class="modal-overlay" @click="closeAbout">
      <div class="modal-content" @click.stop>
        <h2>关于 WebChecker</h2>
        <div class="about-info">
          <p><strong>WebChecker</strong> - 网页批量检查工具</p>
          <p>版本: 0.1.2</p>
          <p>功能: 批量检查网页可访问性、状态码和响应信息</p>
          <p>技术栈: Vue 3 + Tauri + Rust</p>
          <p>版权信息: © 2025 佛子岭日夜加班有限公司. 保留所有权利。</p>
        </div>
        <button class="close-button" @click="closeAbout">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref, computed } from 'vue';
import { useWebChecker } from './composables/useWebChecker.js';
import * as XLSX from 'xlsx';
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';

// 导入组件
import UrlInput from './components/UrlInput.vue';
import ConfigForm from './components/ConfigForm.vue';
import ControlButtons from './components/ControlButtons.vue';
import ResultsTable from './components/ResultsTable.vue';

// 导入样式
import './styles/main.css';
import './styles/components.css';
import './styles/UrlInput.css';
import './styles/ConfigForm.css';
import './styles/ControlButtons.css';
import './styles/ResultsTable.css';

// 菜单状态
const showFileMenu = ref(false);
const showAboutDialog = ref(false);
const isExporting = ref(false);
const isImporting = ref(false);

// 使用组合式函数
const {
  targets,
  userAgent,
  cookie,
  timeout,
  headers,
  results,
  isRunning,
  completed,
  startCheck,
  stopCheck,
  setupListeners,
  clearHistory
} = useWebChecker();

// 计算成功和失败数量 - 使用 results.value
const successCount = computed(() => 
  results.value.filter(r => r.status_code === 200).length
);

const errorCount = computed(() => 
  results.value.filter(r => r.status_code >= 400 || r.error).length
);

// 菜单功能
const toggleFileMenu = () => {
  showFileMenu.value = !showFileMenu.value;
};

const importFile = async () => {
  showFileMenu.value = false;
  
  if (isImporting.value) return;
  
  isImporting.value = true;

  try {
    // 打开文件选择对话框
    const selected = await open({
      multiple: false,
      filters: [{
        name: '文本文件',
        extensions: ['txt', 'csv']
      }],
      title: '选择包含URL列表的文件'
    });

    if (!selected || Array.isArray(selected) && selected.length === 0) {
      // 用户取消了选择
      return;
    }

    const filePath = Array.isArray(selected) ? selected[0] : selected;

    // 读取文件内容
    const fileContent = await readTextFile(filePath);
    
    if (!fileContent.trim()) {
      alert('文件内容为空，请选择包含有效URL的文件。');
      return;
    }

    // 解析文件内容
    const urls = parseFileContent(fileContent);
    
    if (urls.length === 0) {
      alert('文件中未找到有效的URL，请检查文件格式。\n\n支持格式：每行一个URL');
      return;
    }

    // 更新URL输入框
    targets.value = urls.join('\n');
    
    // 显示成功提示
    alert(`✅ 导入成功！\n\n从文件导入了 ${urls.length} 个URL。\n\n文件路径：${filePath}`);
    
  } catch (error) {
    console.error('导入文件失败:', error);
    alert(`❌ 导入失败: ${error.message}\n\n请确保文件存在且格式正确。`);
  } finally {
    isImporting.value = false;
  }
};

// 解析文件内容，提取URL
function parseFileContent(content) {
  const lines = content.split('\n');
  const urls = [];
  
  for (const line of lines) {
    const trimmedLine = line.trim();
    
    // 跳过空行和注释行
    if (!trimmedLine || trimmedLine.startsWith('#')) {
      continue;
    }
    urls.push(trimmedLine);
  }

  return urls;
}

const exportFile = () => {
  showFileMenu.value = false;
  
  // 使用 results.value 来访问实际的数组
  if (results.value.length === 0) {
    alert('暂无数据可导出');
    return;
  }

  isExporting.value = true;

  try {
    // 准备Excel数据 - 使用 results.value
    const excelData = results.value.map(result => ({
      '原始输入': result.original_input || '',
      '协议': getProtocol(result.original_url) || '',
      'Status Code': result.status_code || (result.error ? 'ERROR' : ''),
      'Title': result.title || '',
      'Banner': result.banner || '',
      'Content Length': result.content_length || 0,
      '重定向URL': result.redirect_url || '',
      '错误信息': result.error || ''
    }));

    // 创建工作簿
    const ws = XLSX.utils.json_to_sheet(excelData);
    
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

    // 生成文件名（包含时间戳）
    const now = new Date();
    const timestamp = now.toISOString().replace(/[:.]/g, '-').slice(0, -5);
    const filename = `webchecker-results-${timestamp}.xlsx`;

    // 保存文件
    XLSX.writeFile(wb, filename);

    // 显示成功提示框
    alert(`✅ 导出成功！\n\n文件已保存为：${filename}\n\n文件已自动下载到您的默认下载文件夹中。`);
    
  } catch (error) {
    console.error('Export error:', error);
    alert('❌ 导出失败: ' + error.message);
  } finally {
    isExporting.value = false;
  }
};

// 获取协议函数
function getProtocol(url) {
  if (!url) return '';
  if (url.startsWith('https://')) return 'HTTPS';
  if (url.startsWith('http://')) return 'HTTP';
  return '';
}

const showAbout = () => {
  showAboutDialog.value = true;
};

const closeAbout = () => {
  showAboutDialog.value = false;
};

// 点击其他地方关闭文件菜单
const handleClickOutside = (event) => {
  if (!event.target.closest('.menu-item')) {
    showFileMenu.value = false;
  }
};

// 组件挂载时设置监听器
onMounted(() => {
  setupListeners();
  document.addEventListener('click', handleClickOutside);
});
</script>