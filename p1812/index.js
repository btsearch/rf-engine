import { createRequire } from "module";

const require = createRequire(import.meta.url);
const binding = require("../binding.cjs");

export const calculateP1812 = binding.calculateP1812;
export const calculateP1812Detailed = binding.calculateP1812Detailed;
