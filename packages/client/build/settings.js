import esbuildPluginTsc from "esbuild-plugin-tsc";

export function createBuildSettings(options) {
  return {
    ...options,
    entryPoints: ["src/main.ts"],
    outfile: "dist/main.js",
    bundle: true,
    plugins: [
      esbuildPluginTsc({
        force: true,
      }),
    ],
  };
}
