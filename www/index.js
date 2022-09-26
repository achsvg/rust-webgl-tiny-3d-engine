import { Environment, Camera, Renderer } from "rust-webgl-tiny-3d-engine";

const WIDTH = 800;
const HEIGHT = 600;

function init() {
    const canvas = document.getElementById("canvas");
    canvas.setAttribute("style", "width:" + WIDTH + "px; height:" + HEIGHT + "px");
    var gl = canvas.getContext("webgl2");

    resizeCanvasToDisplaySize(gl.canvas);
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

    const camera = Camera.new(1.0, 2000.0, 50.0, WIDTH / HEIGHT);
    const renderer = Renderer.new(camera);
    const env = Environment.new(renderer);

    function render() {
        env.tick();
        requestAnimationFrame(render);
    }

    requestAnimationFrame(render);
}

function resizeCanvasToDisplaySize(canvas) {
    // Lookup the size the browser is displaying the canvas in CSS pixels.
    const displayWidth = canvas.clientWidth;
    const displayHeight = canvas.clientHeight;

    // Check if the canvas is not the same size.
    const needResize = canvas.width !== displayWidth ||
        canvas.height !== displayHeight;

    if (needResize) {
        // Make the canvas the same size
        canvas.width = displayWidth;
        canvas.height = displayHeight;
    }

    return needResize;
}

init();


