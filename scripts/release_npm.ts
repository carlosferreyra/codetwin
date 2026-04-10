// @ts-nocheck
import { mkdir } from "node:fs/promises";
import { join } from "node:path";

class PackageMetadata {
  constructor(
    public readonly rawName: string,
    public readonly version: string,
    public readonly description: string,
    public readonly repository: string,
    public readonly license: string,
  ) {}

  get safeName(): string {
    return this.rawName.toLowerCase().replace(/[^a-z0-9-_]/g, "");
  }

  get repositoryUrl(): string {
    let url = this.repository;
    if (url.startsWith("https://github.com/")) {
      url = `git+${url}`;
    }
    return url.endsWith(".git") ? url : `${url}.git`;
  }

  static async fromCargo(): Promise<PackageMetadata> {
    const content = await Bun.file("Cargo.toml").text();
    const extract = (key: string) => {
      const match = content.match(new RegExp(`${key}\\s*=\\s*"([^"]+)"`));
      return match ? match[1] : "";
    };

    const name = extract("name");
    const version = extract("version");

    if (!name || !version) {
      throw new Error("Cargo.toml is missing name/version.");
    }

    return new PackageMetadata(
      name,
      version,
      extract("description") || "Rust CLI wrapper",
      extract("repository"),
      extract("license") || "MIT",
    );
  }
}

const Templates = {
  packageJson: (meta: PackageMetadata) => ({
    name: meta.safeName,
    version: meta.version,
    description: meta.description,
    license: meta.license,
    type: "module",
    bin: { [meta.safeName]: `./bin/${meta.safeName}.cjs` },
    files: ["bin", "README.md"],
    repository: { type: "git", url: meta.repositoryUrl },
    homepage: meta.repository,
    publishConfig: { access: "public" },
    engines: { node: ">=18" },
  }),

  cliWrapper: (meta: PackageMetadata) =>
    `
import { spawnSync } from "node:child_process";
import { platform, homedir } from "node:os";
import { join } from "node:path";
import { existsSync } from "node:fs";

const BIN_NAME = "${meta.rawName}";
const REPOSITORY = "${meta.repository}".replace(/\\/$/, "");
const VERSION = "${meta.version}";

function bootstrapBinary() {
  const tag = "v" + VERSION;
  const currentPlatform = platform();

  if (currentPlatform === "win32") {
    const url = \`\${REPOSITORY}/releases/download/\${tag}/\${BIN_NAME}-installer.ps1\`;
    spawnSync("powershell", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", "iwr -useb '" + url + "' | iex"], { stdio: "inherit" });
  } else {
    const url = \`\${REPOSITORY}/releases/download/\${tag}/\${BIN_NAME}-installer.sh\`;
    spawnSync("sh", ["-c", "curl -LsSf '" + url + "' | sh"], { stdio: "inherit" });
  }
}

const args = process.argv.slice(2);
const isWin = platform() === "win32";

// Use absolute path to avoid the npm shim causing infinite recursion.
const binPath = join(
  homedir(),
  ".cargo",
  "bin",
  isWin ? \`\${BIN_NAME}.exe\` : BIN_NAME
);

if (!existsSync(binPath)) {
  process.stderr.write(\`Binary '\${BIN_NAME}' not found at \${binPath}. Installing...\\n\`);
  bootstrapBinary();
}

const result = spawnSync(binPath, args, {
  stdio: "inherit",
  shell: isWin,
});

process.exit(result.status ?? 0);
`.trim(),
};

async function main() {
  try {
    const meta = await PackageMetadata.fromCargo();
    const outDir = ".release/npm";
    const srcDir = join(outDir, "src");
    const binDir = join(outDir, "bin");

    await mkdir(srcDir, { recursive: true });
    await mkdir(binDir, { recursive: true });

    await Bun.write(
      join(outDir, "package.json"),
      JSON.stringify(Templates.packageJson(meta), null, 2),
    );

    const entryPath = join(srcDir, "cli.ts");
    await Bun.write(entryPath, Templates.cliWrapper(meta));

    console.log(`Bundling ${meta.safeName}...`);

    const buildResult = await Bun.build({
      entrypoints: [entryPath],
      outdir: binDir,
      target: "node",
      format: "cjs",
      naming: `${meta.safeName}.cjs`,
      minify: true,
      sourcemap: "none",
      banner: "#!/usr/bin/env node",
      compile: false,
    });

    if (!buildResult.success) {
      console.error("Build failed:", buildResult.logs);
      process.exit(1);
    }

    const readme = Bun.file("README.md");
    if (await readme.exists()) {
      await Bun.write(join(outDir, "README.md"), readme);
    }

    console.log(`Generated ${meta.safeName} v${meta.version} npm wrapper in .release/npm/`);
  } catch (err) {
    console.error("Error:", err instanceof Error ? err.message : err);
    process.exit(1);
  }
}

main();
