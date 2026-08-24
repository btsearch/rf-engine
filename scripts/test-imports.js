import { calculateCoverage, calculateCoverageWithTerrain } from "../coverage/index.js";
import { calculateLinkBudget } from "../link-budget/index.js";
import { calculateP1812, calculateP1812Detailed } from "../p1812/index.js";

const exportsToCheck = {
  calculateCoverage,
  calculateCoverageWithTerrain,
  calculateLinkBudget,
  calculateP1812,
  calculateP1812Detailed,
};

for (const [name, exportedValue] of Object.entries(exportsToCheck)) {
  if (typeof exportedValue !== "function") throw new TypeError(`Expected ${name} to be a function`);
}
