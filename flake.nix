{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";

    openrazer = {
      url = "github:openrazer/openrazer";
      flake = false;
    };
    razer-cli = {
      url = "github:lolei/razer-cli";
      flake = false;
    };
  };

  outputs = inputs@{ self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        myPythonPackages = with pkgs.python3Packages; rec {
          openrazer-daemon = buildPythonPackage {
            name = "openrazer";
            src = "${inputs.openrazer}/daemon";
            propagatedBuildInputs =
              [ daemonize dbus-python pygobject3 pyudev setproctitle ];
          };
          openrazer = buildPythonPackage {
            name = "openrazer";
            src = "${inputs.openrazer}/pylib";
            propagatedBuildInputs = [ dbus-python numpy openrazer-daemon ];
          };
          razer-cli = buildPythonApplication {
            pname = "razer-cli";
            version = "2.2.0";
            src = inputs.razer-cli;
            propagatedBuildInputs = [ openrazer ];
            setuptoolsCheckPhase = "export HOME=$(mktemp -d)";
          };
        };

        craneLib = crane.lib.${system};
        my-crate = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          buildInputs = with pkgs; [ pkg-config udev libinput ];
        };
        wrapped-crate = my-crate.overrideAttrs (package: { });
      in {
        checks = { inherit my-crate; };

        packages.default = wrapped-crate;

        apps.default = flake-utils.lib.mkApp { drv = wrapped-crate; };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [ myPythonPackages.razer-cli ];
        };
      });
}
