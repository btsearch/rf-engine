import { createRequire } from "module";

const require = createRequire(import.meta.url);
const binding = require("../binding.cjs");

export const calculateCoverage = binding.calculateCoverage;
export const calculateCoverageWithTerrain = binding.calculateCoverageWithTerrain;
