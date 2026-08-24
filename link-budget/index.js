import { createRequire } from "module";

const require = createRequire(import.meta.url);
const binding = require("../binding.cjs");

export const calculateLinkBudget = binding.calculateLinkBudget;
