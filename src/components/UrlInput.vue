<template>
  <div class="input-section">
    <h3>目标网址</h3>
    <textarea 
      :value="modelValue"  
      @input="$emit('update:modelValue', $event.target.value)" 
      placeholder="请输入待访问的网址..."
      class="targets-input"
      rows="6"
      :disabled="disabled"
    ></textarea>
    <div class="input-hint">
      提示：如果没有指定协议（http/https），将同时尝试两种协议
    </div>
  </div>
</template>


<script setup>
defineProps({
  modelValue: {
    type: String,
    required: true
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

defineEmits(['update:modelValue']);
</script>

<style scoped>
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

.targets-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.input-hint {
  font-size: 12px;
  color: #6c757d;
  margin-top: 5px;
  font-style: italic;
}
</style>
