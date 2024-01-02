const appWindow = window.__TAURI__.window;

window.onload = async () => {
    await appWindow.setDecorations(true);
};