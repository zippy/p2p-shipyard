{ lib, rustPlatform, fetchFromGitHub, makeWrapper, zig }:

rustPlatform.buildRustPackage rec {
  pname = "cargo-zigbuild";
  version = "0.18.4";

  src = fetchFromGitHub {
    owner = "guillemcordoba";
    repo = pname;
    rev = "10f111b83157cc63ecd105ef77457a3880dfdfa8";
    hash = "sha256-x9jmp1n0wMXGfeliRvBnx0/pDpfZUXd9/wtyY6Z10Q8==";
  };

  cargoHash = "sha256-SEcmhtNOtv6YQSnSX2D0jKdb5/gLXQcPMUA2RovYPqk=";

  nativeBuildInputs = [ makeWrapper ];

  postInstall = ''
    wrapProgram $out/bin/cargo-zigbuild \
      --prefix PATH : ${zig}/bin
  '';

  meta = with lib; {
    description = "A tool to compile Cargo projects with zig as the linker";
    mainProgram = "cargo-zigbuild";
    homepage = "https://github.com/messense/cargo-zigbuild";
    changelog = "https://github.com/messense/cargo-zigbuild/releases/tag/v${version}";
    license = licenses.mit;
    maintainers = with maintainers; [ figsoda ];
  };
}