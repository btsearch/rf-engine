import { renameSync, existsSync, unlinkSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

const src = resolve(root, "index.js");
const dest = resolve(root, "binding.cjs");

if (existsSync(src)) {
  try {
    if (existsSync(dest)) unlinkSync(dest);
  } catch {}
  renameSync(src, dest);
  console.log("renamed index.js -> binding.cjs");
}
