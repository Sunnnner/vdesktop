// worker.js
import { invoke } from '@tauri-apps/api/core'

// 监听主线程发送的消息
onmessage = async function(event: any) {
    await invoke('boot_screen', { name: event.name })
    postMessage('done');
  }
  