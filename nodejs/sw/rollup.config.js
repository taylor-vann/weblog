import { nodeResolve } from "@rollup/plugin-node-resolve";

const initialBundle = {
  input: "./dist/mod.js",
  output: {
    file: "../../www/sw.js",
    format: "es",
  },
  plugins: [nodeResolve()],
};

export default [initialBundle];
