'use strict';
document.addEventListener('keydown', (event) => {
    const keyName = event.key;
    if (keyName === 'h') {
        var x = document.getElementById("obj");
        if (x.style.display === "none") {
            x.style.display = "block";
        } else {
            x.style.display = "none";
        }
    }
}, false);

function hide() {
    var x = document.getElementById("draggable");
    if (x.style.display === "none") {
        x.style.display = "block";
    } else {
        x.style.display = "none";
    }
}

function hide_all() {
    var x = document.getElementById("obj");
    if (x.style.display === "none") {
        x.style.display = "block";
    } else {
        x.style.display = "none";
    }
}
$('#button').click(function(e) {
    e.preventDefault(); //to prevent standard click event
    $('#wizard').toggle();
});
$(function() {
    $("#draggable").draggable();
});
$(function() {
    $("#obj").draggable();
});

var Module = {};
var __cargo_web = {};
Object.defineProperty(Module, 'canvas', {
    get: function() {
        if (__cargo_web.canvas) {
            return __cargo_web.canvas;
        }
        var canvas = document.createElement('canvas');
        document.querySelector('body').appendChild(canvas);
        __cargo_web.canvas = canvas;
        return canvas;
    }
});