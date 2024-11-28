// worker.ts
import { invoke } from '@tauri-apps/api/tauri';

// 监听主线程发送的消息
onmessage = async function(event) {
    await invoke('boot_screen', { name: event.name })
    postMessage('done');
  }