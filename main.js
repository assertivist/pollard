'use strict'
const electron = require('electron')

const app = electron.app
const BrowserWindow = electron.BrowserWindow

let mainWindow

app.on('ready', function() {
	mainWindow = new BrowserWindow({
		width:1024,
		height:768
	})

	mainWindow.loadURL(`file://${__dirname }/index.html`)
	mainWindow.webContents.openDevTools()

	mainWindow.on('closed', function() { mainWindow = null })
});
