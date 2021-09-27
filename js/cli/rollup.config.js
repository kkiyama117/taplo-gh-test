import rust from "@wasm-tool/rollup-plugin-rust";
import typescript from "rollup-plugin-typescript2";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import { terser } from "rollup-plugin-terser";

export default {
  input: {
    cli: "src/cli.ts",
  },
  output: {
    sourcemap: false,
    format: "cjs",
    dir: "dist",
  },
  plugins: [
    rust({
      debug: process.env["RELEASE"] !== "true",
      nodejs: true,
      inlineWasm: process.env["SEPARATE_WASM"] !== "true",
      cargoArgs: ["--features=nightly"]
    }),
    resolve({ jsnext: true, preferBuiltins: true }),
    commonjs({ include: ["src/*.ts", "node_modules/**"] }),
    typescript(),
    terser()
  ],
};
