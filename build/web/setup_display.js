// This function runs after the page has loaded.
// It checks that the <canvas> element has been loaded into the body,
// and if so it moves the <canvas> into the <div id="wasm_window"> node.
// The init_display interval them cancelled.
(() => {
    const check_interval = 100; // ms
    let init_display = setInterval(function () {
        let canvas = document.body.getElementsByTagName("canvas");

        if (canvas.length > 0) {
            var fragment = document.createDocumentFragment();
            fragment.appendChild(canvas[0]);
            document.getElementById("wasm-window").appendChild(fragment);
            clearInterval(init_display);
        }
    }, check_interval);
})();
