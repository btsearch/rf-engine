async function main() {
  const requiredFiles = [
    "binding.cjs",
    "coverage/index.d.ts",
    "coverage/index.js",
    "link-budget/index.d.ts",
    "link-budget/index.js",
    "p1812/index.d.ts",
    "p1812/index.js",
  ];
  let input = "";

  for await (const chunk of process.stdin) input += chunk;

  const [packResult] = JSON.parse(input);
  const packedFiles = new Set(
    packResult.files.map(({ path }) => path.split(String.fromCharCode(92)).join("/")),
  );
  const missingFiles = requiredFiles.filter((path) => !packedFiles.has(path));

  if (missingFiles.length > 0)
    throw new Error("Package is missing required files: " + missingFiles.join(", "));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
