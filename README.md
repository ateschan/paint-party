# paint-party [![Cargo Build & Test](https://github.com/ateschan/paint-party/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/ateschan/paint-party/actions/workflows/test.yml) ![GitHub commits since latest release](https://img.shields.io/github/commits-since/ateschan/paint-party/latest?include_prereleases) ![](https://tokei.rs/b1/github/ateschan/paint-party)

## Installation
1. Download the latest release at https://github.com/ateschan/paint-party/tree/v0.1.3
2. Join a server or spawn a local one, see https://github.com/ateschan/paint-party-server for compilation

## Compile from source 
### Prerequisites
You need rust installed, follow directions on https://rustup.rs/

### Linux 
If you are using linux you will require additional dependencies
```
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
 pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

### Windows 
Openssl is a required dependency of quad-net, to compile on windows you need perl installed
https://strawberryperl.com/
<!-- USAGE EXAMPLES -->
## Usage
1. Enter the designated server in main menu
2. Enter the correspnding password in toolbar, if there is none set skip this step
3. Select a canvas or "room" by entering a number between 0 and 9999
4. Select a color and start painting!

<!-- ROADMAP -->
## Roadmap
- [x] Eraser tool
- [X] Particles
- [X] Hotkey support
- [X] Chat
- [X] Graceful (ish) error handling
- [X] Chromatic Modulator
- [X] Size Oscillator
- [X] Arrow movment
- [ ] Tail
- [ ] Spitter 
- [ ] Blast
- [ ] Orbit
- [ ] Trig Tool

See the [open issues](https://github.com/ateschan/paint-party/issues) for a full list of proposed features (and known issues).

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the GPLv2 License. See `LICENSE.txt` for more information.
