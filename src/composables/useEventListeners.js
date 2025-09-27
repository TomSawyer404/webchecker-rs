// 事件监听逻辑
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";

export function useEventListeners() {
    const unlistenResult = ref(null);
    const unlistenComplete = ref(null);

    async function setupEventListeners(onResult, onComplete) {
        // 监听检查结果事件
        unlistenResult.value = await listen('check_result', (event) => {
            const result = event.payload;
            onResult(result);
        });

        // 监听检查完成事件
        unlistenComplete.value = await listen('check_complete', () => {
            onComplete();
        });
    }

    function cleanupEventListeners() {
        if (unlistenResult.value) {
            unlistenResult.value();
            unlistenResult.value = null;
        }
        if (unlistenComplete.value) {
            unlistenComplete.value();
            unlistenComplete.value = null;
        }
    }

    onMounted(() => {
        // 在组件挂载时设置监听器
    });

    onUnmounted(() => {
        cleanupEventListeners();
    });

    return {
        setupEventListeners,
        cleanupEventListeners
    };
}