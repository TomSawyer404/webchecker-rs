<template>
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
          :disabled="disabled"
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
          :disabled="disabled"
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
          :disabled="disabled"
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
          :disabled="disabled"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  userAgent: {
    type: String,
    required: true
  },
  cookie: {
    type: String,
    required: true
  },
  timeout: {
    type: Number,
    required: true
  },
  headers: {
    type: String,
    required: true
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:userAgent', 'update:cookie', 'update:timeout', 'update:headers']);

const userAgent = computed({
  get: () => props.userAgent,
  set: (value) => emit('update:userAgent', value)
});

const cookie = computed({
  get: () => props.cookie,
  set: (value) => emit('update:cookie', value)
});

const timeout = computed({
  get: () => props.timeout,
  set: (value) => emit('update:timeout', value)
});

const headers = computed({
  get: () => props.headers,
  set: (value) => emit('update:headers', value)
});
</script>
