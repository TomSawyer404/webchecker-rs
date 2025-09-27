// 主要业务逻辑
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useEventListeners } from './useEventListeners.js';
import { parseUrlList, buildConfig } from '../utils/index.js';

export function useWebChecker() {
    // 响应式数据
    const targets = ref("");
    const userAgent = ref("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");
    const cookie = ref("");
    const timeout = ref(30);
    const headers = ref("");
    const results = ref([]);
    const isRunning = ref(false);
    const completed = ref(false);

    // 事件监听
    const { setupEventListeners, cleanupEventListeners } = useEventListeners();

    // 设置事件监听器
    // const setupListeners = () => {
    //     setupEventListeners(
    //         (result) => {
    //             results.value.push(result);
    //         },
    //         () => {
    //             completed.value = true;
    //             isRunning.value = false;
    //         }
    //     );
    // };

    const setupListeners = async () => {  // 添加async
        try {
            await setupEventListeners(
                (result) => {
                    results.value.push(result);
                },
                () => {
                    completed.value = true;
                    isRunning.value = false;
                }
            );
        } catch (error) {
            console.error("设置事件监听器失败:", error);
        }
    };




    // 开始检查
    async function startCheck() {
        if (!targets.value.trim()) {
            alert("请输入待访问的网址");
            return;
        }

        // 重置状态
        isRunning.value = true;
        completed.value = false;
        results.value = [];

        try {
            const configObj = buildConfig(userAgent.value, cookie.value, timeout.value, headers.value);
            const urlList = parseUrlList(targets.value);

            await invoke("batch_check_urls", {
                urls: urlList,
                config: configObj
            });

        } catch (error) {
            console.error("访问出错:", error);
            alert("访问过程中出现错误: " + error.toString());
            isRunning.value = false;
        }
    }

    // 停止检查
    function stopCheck() {
        isRunning.value = false;
        completed.value = true;
    }

    return {
        // 状态
        targets,
        userAgent,
        cookie,
        timeout,
        headers,
        results,
        isRunning,
        completed,

        // 方法
        startCheck,
        stopCheck,
        setupListeners
    };
}