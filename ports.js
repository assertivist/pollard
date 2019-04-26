'use strict'
var n = document.getElementById("elm");
var app = Elm.Main.init({
	node: n,
	flags: undefined
});

var rpc = {
	invoke: function(arg) { window.external.invoke(JSON.stringify(arg)); },
	init : function() { rpc.invoke({cmd : 'init'}); },
	log: function() {
	    var s = '';
	    for (var i = 0; i < arguments.length; i++) {
	      if (i != 0) {
		s = s + ' ';
	      }
	      s = s + JSON.stringify(arguments[i]);
	    }
	    rpc.invoke({cmd : 'log', text : s});
	},
	chat: function(str) {
		rpc.invoke({cmd: 'sendChat', text: str});
	}
}

app.ports.netWrite.subscribe(rpc.chat);