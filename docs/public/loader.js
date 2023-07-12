async function impl_run() {
    let register_plugin = function (importObject) {
        importObject.env.go_to_location = function () {
            window.location.href = "/orange-museum/basement/message";
        }
    }
    miniquad_add_plugin({register_plugin});
    load("/orange-museum/public/basement.wasm");
};

impl_run();