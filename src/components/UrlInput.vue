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
