import Editor from "./components/Editor";
import { render } from "preact";

import init, { greet } from "../pkg";

async function main() {
    await init("../pkg/walrust_runtime_bg.wasm");
    greet("World");

    render(Editor(), document.body);
}

main();
