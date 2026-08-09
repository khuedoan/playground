{
  description = "Prime Agent test environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    prime-agent-src = {
      url = "github:PrimeIntellect-ai/prime-agent/main";
      flake = false;
    };
  };

  outputs =
    { nixpkgs, prime-agent-src, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      package =
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          buildNpmPackage = pkgs.buildNpmPackage.override {
            nodejs = pkgs.nodejs_22;
          };
        in
        buildNpmPackage {
          pname = "prime-agent";
          version = "0.7.1-unstable";
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
              packages/coding-agent/{CHANGELOG.md,README.md,dist,docs,examples,package.json,postinstall.cjs,skills} \
              node_modules \
              "$package_dir/"

            # npm workspaces leave links to packages that are not installed in the output.
            rm -f "$package_dir"/node_modules/@earendil-works/{pi-agent-core,pi-ai,pi-coding-agent,pi-tui}
            rm -f "$package_dir"/node_modules/pi-extension-{custom-provider-anthropic,custom-provider-gitlab-duo,sandbox,with-deps}

            makeWrapper ${pkgs.nodejs_22}/bin/node "$out/bin/prime-agent" \
              --add-flags "$package_dir/dist/bundle/cli.js" \
              --set PI_PACKAGE_DIR "$package_dir" \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  pkgs.uv
                  pkgs.python311
                ]
              }

            runHook postInstall
          '';
        };
    in
    {
      packages = forAllSystems (system: {
        default = package system;
      });

      devShells = forAllSystems (system: {
        default = nixpkgs.legacyPackages.${system}.mkShell {
          packages = [ (package system) ];
        };
      });
    };
}
