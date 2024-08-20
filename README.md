# paint-party client
[![Cargo Build & Test](https://github.com/ateschan/paint-party/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/ateschan/paint-party/actions/workflows/test.yml)
<!-- ABOUT THE PROJECT -->
## About The Project
Paint party is a small capstone project developed for my Computer Science degree at NVC

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites
You need rust installed, follow directions on https://rustup.rs/

### Installation

1. Clone the Repo
   ```sh
   git clone git@github.com:ateschan/paint-party.git
   ```
2. Change Dir
   ```sh
   cd paint-party
   ```
3. Build Project
   ```sh
   cargo build --release
   ```
4. Run
   ```sh
   cd target/release && ./paint-party
   ```

<!-- USAGE EXAMPLES -->
## Usage
1. Enter the designated server ip with the port
2. Enter the correspnding password, if there is none set skip this step
3. Select a canvas or "room" by entering a number between 0 and 9999
4. Select a color and start painting!

<!-- ROADMAP -->
## Roadmap

- [ ] Eraser tool
- [ ] Compile down to webassembly

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
