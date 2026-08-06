# Prime Agent Nix environment

This experiment tests [Prime Agent](https://github.com/PrimeIntellect-ai/prime-agent) without executing the upstream shell installer. Nix builds the Node.js application and provides the `uv` and Python tools used by Prime Agent’s normal kernel bootstrap.

## Run Prime Agent

Enter the development shell:

```sh
nix develop
prime-agent
```

On first launch, use `/login` to configure a provider. On first IPython use, Prime Agent creates its normal writable kernel environment under `~/.prime` and installs its runtime and Python-backed skills, matching the upstream installation behavior.

You can also run one command without entering a shell:

```sh
nix develop -c prime-agent --version
```

The flake pins the Prime Agent source and Node.js dependencies in `flake.lock` and `package-lock.json`. It does not execute the upstream `curl | sh` command. The first Nix build downloads the pinned source and dependency archives; Prime Agent manages its writable Python kernel environment at runtime as upstream does.

> [!WARNING]
> Prime Agent runs model-generated code and commands with your user permissions. Test it in a disposable repository or another restricted environment.

## Result

`nix develop -c prime-agent --version` returns `0.7.0` on `aarch64-darwin`.

Primary metric: command availability, measured as pass or fail. Pass is better. The result is pass.
