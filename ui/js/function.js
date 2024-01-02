const appWindow = window.__TAURI__.window;

function decorate() {
    window.onload = async () => {
        await appWindow.setDecorations(true);
    };
}

decorate