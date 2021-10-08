import { app, BrowserWindow } from "electron";
import path from "path";

async function createWindow() {
    const win = new BrowserWindow({
        width: 1280,
        height: 720,
        autoHideMenuBar: true,
        webPreferences: {
            devTools: true
        }
    });

    win.loadFile(path.join(__dirname, "../public/index.html"));
}

app.whenReady().then(() => {
    createWindow();

    app.on('activate', function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
});

app.on('window-all-closed', function () {
    if (process.platform !== 'darwin') app.quit()
})