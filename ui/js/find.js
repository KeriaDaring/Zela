const invoke = window.__TAURI__.invoke;
console.log('加载到了')
self.addEventListener("message", async (event) => {
    await invoke("search", {target: event.data}).then(async () => {
        console.log("开始工作")
        let times = 0;
        while (1) {
            let list = await invoke("get_file");
            // if (!list) times++;
            if (times > 1000) break;
            console.log(list)
            self.postMessage(list)
            times = 0;
        }
    })
})