require("esbuild").build({
    entryPoints: ["src/client.ts"],
    bundle: true,
    outfile: "public/bundle.js"
}).catch(() => process.exit(1));