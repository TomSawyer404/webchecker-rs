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
