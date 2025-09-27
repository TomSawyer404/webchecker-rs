<template>
  <div class="app-container">
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
</template>

<script setup>
import { onMounted } from 'vue';
import { useWebChecker } from './composables/useWebChecker.js';

// 导入组件
import UrlInput from './components/UrlInput.vue';
import ConfigForm from './components/ConfigForm.vue';
import ControlButtons from './components/ControlButtons.vue';
import ResultsTable from './components/ResultsTable.vue';

// 导入样式
import './styles/main.css';
import './styles/components.css';

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
  setupListeners
} = useWebChecker();

// 组件挂载时设置监听器
onMounted(() => {
  setupListeners();
});
</script>