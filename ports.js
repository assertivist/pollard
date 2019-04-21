'use strict'
var n = document.getElementById("elm");
var app = Elm.Main.init({
	node: n,
	flags: undefined
});

app.ports.netWrite.subscribe(function (str) {
	//console.log("received: ", str);
	external.invoke('test', str);
});