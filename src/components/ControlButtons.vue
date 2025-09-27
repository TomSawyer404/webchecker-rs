<template>
  <div class="button-group">
    <button 
      @click="onStart" 
      :disabled="disabled"
      class="start-button"
    >
      {{ isRunning ? '访问中...' : '开始访问' }}
    </button>
    <button 
      v-if="isRunning"
      @click="onStop"
      class="stop-button"
    >
      停止
    </button>
  </div>
</template>

<script setup>
defineProps({
  isRunning: {
    type: Boolean,
    default: false
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['start', 'stop']);

function onStart() {
  emit('start');
}

function onStop() {
  emit('stop');
}
</script>

<style scoped>
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
</style>