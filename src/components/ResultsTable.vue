<template>
  <div class="results-container">
    <div class="results-header">
      <h3>访问结果</h3>
    </div>
    
    <div class="results-content">
      <div v-if="results.length === 0 && !isRunning" class="no-results">
        暂无访问结果，点击"开始访问"按钮开始检查
      </div>
      <div v-else class="results-table-container">
        <table class="results-table">
          <thead>
            <tr>
              <th @click="sortBy('original_input')" class="sortable-header">
                原始输入
                <span class="sort-indicator" v-if="sortField === 'original_input'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
              <th @click="sortBy('protocol')" class="sortable-header">
                协议
                <span class="sort-indicator" v-if="sortField === 'protocol'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
              <th @click="sortBy('status_code')" class="sortable-header">
                Status Code
                <span class="sort-indicator" v-if="sortField === 'status_code'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
              <th @click="sortBy('title')" class="sortable-header">
                Title
                <span class="sort-indicator" v-if="sortField === 'title'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
              <th @click="sortBy('banner')" class="sortable-header">
                Banner
                <span class="sort-indicator" v-if="sortField === 'banner'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
              <th @click="sortBy('content_length')" class="sortable-header">
                Content Length
                <span class="sort-indicator" v-if="sortField === 'content_length'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
              <th @click="sortBy('redirect_url')" class="sortable-header">
                重定向URL
                <span class="sort-indicator" v-if="sortField === 'redirect_url'">
                  {{ sortDirection === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="(result, index) in sortedResults" 
              :key="index"
              :class="getRowClass(result)"
            >
              <td class="original-input-cell">
                <span 
                  class="original-input-text clickable-url"
                  @dblclick="openUrl(result.original_input)"
                  :title="'双击打开: ' + result.original_input"
                >{{ result.original_input }}</span>
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
              <td class="redirect-cell">
                <span 
                  v-if="result.redirect_url"
                  class="redirect-url-text clickable-url"
                  @dblclick="openUrl(result.redirect_url)"
                  :title="'双击打开: ' + result.redirect_url"
                >{{ result.redirect_url }}</span>
                <span v-else>无</span>
              </td>
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

const sortField = ref('');
const sortDirection = ref('asc');

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

// 双击打开URL的函数
function openUrl(url) {
  if (!url) return;
  
  try {
    // 确保URL有协议前缀
    let fullUrl = url;
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      fullUrl = 'http://' + url;
    }
    
    console.log('正在打开URL:', fullUrl);
    
    // 方法1: 使用更可靠的window.open方式
    const newWindow = window.open('', '_blank');
    if (newWindow) {
      newWindow.location.href = fullUrl;
    } else {
      // 方法2: 如果被阻止，使用a标签方式
      const link = document.createElement('a');
      link.href = fullUrl;
      link.target = '_blank';
      link.rel = 'noopener noreferrer';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    }
    
  } catch (error) {
    console.error('打开URL失败:', error);
    // 方法3: 最后尝试直接设置location
    try {
      window.location.href = fullUrl;
    } catch (fallbackError) {
      alert('❌ 无法打开URL。请检查浏览器是否阻止了弹出窗口，或尝试手动复制链接到浏览器中打开。');
    }
  }
}

// 计算排序后的结果
const sortedResults = computed(() => {
  if (!sortField.value) return props.results;
  
  return [...props.results].sort((a, b) => {
    let aValue = getSortValue(a, sortField.value);
    let bValue = getSortValue(b, sortField.value);
    
    // 处理空值
    if (aValue === null || aValue === undefined) aValue = '';
    if (bValue === null || bValue === undefined) bValue = '';
    
    // 数字类型排序
    if (typeof aValue === 'number' && typeof bValue === 'number') {
      return sortDirection.value === 'asc' ? aValue - bValue : bValue - aValue;
    }
    
    // 字符串类型排序
    const aStr = String(aValue).toLowerCase();
    const bStr = String(bValue).toLowerCase();
    
    if (sortDirection.value === 'asc') {
      return aStr.localeCompare(bStr);
    } else {
      return bStr.localeCompare(aStr);
    }
  });
});

// 获取排序字段的值
function getSortValue(result, field) {
  switch (field) {
    case 'protocol':
      return getProtocol(result.original_url);
    case 'original_input':
      return result.original_input || '';
    case 'status_code':
      return result.status_code || 0;
    case 'title':
      return result.title || '';
    case 'banner':
      return result.banner || '';
    case 'content_length':
      return result.content_length || 0;
    case 'redirect_url':
      return result.redirect_url || '';
    default:
      return '';
  }
}

// 排序函数
function sortBy(field) {
  if (sortField.value === field) {
    // 切换排序方向
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc';
  } else {
    // 设置新的排序字段，默认升序
    sortField.value = field;
    sortDirection.value = 'asc';
  }
}

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