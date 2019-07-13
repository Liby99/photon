const Photon = require("../native");

// console.log(Photon);

// document.getElementById("log").innerHTML += "<br />" + Photon.another_hello();

const canvas = document.getElementById("canvas");
const context = canvas.getContext('2d');

const img = context.createImageData(50, 50);
const imgData = img.data;

Photon.render(imgData);

context.putImageData(img, 0, 0);