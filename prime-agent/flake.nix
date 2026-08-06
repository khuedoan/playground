{
  description = "Prime Agent test environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    prime-agent-src = {
      url = "github:PrimeIntellect-ai/prime-agent/c22549a37b73cc603c6f0d202517cb0ca856c7d3";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, prime-agent-src }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          buildNpmPackage = pkgs.buildNpmPackage.override {
            nodejs = pkgs.nodejs_22;
          };
        in {
          default = buildNpmPackage {
            pname = "prime-agent";
            version = "0.7.0-unstable-2026-08-05";
            src = prime-agent-src;

            postPatch = ''
              cp ${./package-lock.json} package-lock.json
              substituteInPlace packages/ai/package.json \
                --replace-fail \
                  '"build": "npm run generate-models && tsgo -p tsconfig.build.json"' \
                  '"build": "tsgo -p tsconfig.build.json"'
            '';

            npmDepsHash = "sha256-8KyfC+AiEhyMmUmeq/3b5WIjWK6S0SMRp6JJyyZeJ+Q=";
            npmDepsFetcherVersion = 2;
            npmBuildScript = "build";

            nativeBuildInputs = [
              pkgs.makeWrapper
              pkgs.pkg-config
              pkgs.python311
            ];
            buildInputs = [
              pkgs.cairo
              pkgs.giflib
              pkgs.libjpeg
              pkgs.libpng
              pkgs.librsvg
              pkgs.pango
              pkgs.pixman
              pkgs.zeromq
            ];

            installPhase = ''
              runHook preInstall

              package_dir="$out/lib/prime-agent"
              mkdir -p "$package_dir" "$out/bin"
              cp -R \
                packages/coding-agent/CHANGELOG.md \
                packages/coding-agent/README.md \
                packages/coding-agent/dist \
                packages/coding-agent/docs \
                packages/coding-agent/examples \
                packages/coding-agent/package.json \
                packages/coding-agent/postinstall.cjs \
                packages/coding-agent/skills \
                node_modules \
                "$package_dir/"

              rm -f \
                "$package_dir/node_modules/@earendil-works/pi-agent-core" \
                "$package_dir/node_modules/@earendil-works/pi-ai" \
                "$package_dir/node_modules/@earendil-works/pi-coding-agent" \
                "$package_dir/node_modules/@earendil-works/pi-tui" \
                "$package_dir/node_modules/pi-extension-custom-provider-anthropic" \
                "$package_dir/node_modules/pi-extension-custom-provider-gitlab-duo" \
                "$package_dir/node_modules/pi-extension-sandbox" \
                "$package_dir/node_modules/pi-extension-with-deps"

              makeWrapper ${pkgs.nodejs_22}/bin/node "$out/bin/prime-agent" \
                --add-flags "$package_dir/dist/bundle/cli.js" \
                --set PI_PACKAGE_DIR "$package_dir" \
                --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.uv pkgs.python311 ]}

              runHook postInstall
            '';

            meta = {
              description = "Self-improving RLM coding and research agent";
              homepage = "https://github.com/PrimeIntellect-ai/prime-agent";
              license = nixpkgs.lib.licenses.mit;
              mainProgram = "prime-agent";
              platforms = systems;
            };
          };
        });

      devShells = forAllSystems (system: {
        default = nixpkgs.legacyPackages.${system}.mkShell {
          packages = [ self.packages.${system}.default ];
        };
      });
    };
}
