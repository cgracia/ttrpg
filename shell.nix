{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config

    # X11
    libx11
    libxcursor
    libxrandr
    libxi

    # Audio
    alsa-lib

    # udev
    udev

    # Vulkan
    vulkan-loader
    vulkan-headers

    # Windowing
    libxkbcommon
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
      pkgs.libx11
      pkgs.libxcursor
      pkgs.libxrandr
      pkgs.libxi
      pkgs.alsa-lib
      pkgs.udev
      pkgs.vulkan-loader
      pkgs.libxkbcommon
    ]}:$LD_LIBRARY_PATH
  '';
}
