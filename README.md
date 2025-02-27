<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">LegoLend</h3>

  <p align="center">
    A composable lending protocol built on Solana, and inspired by Morpho Blue
    <br />
    <a href="https://github.com/mgnfy-view/lego-lend/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/mgnfy-view/lego-lend/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

LegoLend is a composable lending protocol built on Solana. It is heavily inspired by Morpho Blue, one of the hottest lending protocols from 2024.

LegoLend allows you to create lending pools for any pair of assets by attaching your custom oracles and interest rate models. It represents a user's supplied and borrowed assets in the form of shares, which enables any bad debt to be spread among the lenders. Due to its individual pool approach, LegoLend provides a very high liquidation loan to value threshold and, consequently, generates higher interest for lenders.

Lenders are not expected to interact directly with LegoLend, instead they should use allocation strategy vaults attested by LegoLend to split their deposits accross multiple lending pools. This also activates their deposit to be eligible for any additional points/incentives/rewards provided by the allocation strategy vault manager.

### Built With

-   Rust
-   Anchor
-   Solana
-   Yarn

<!-- GETTING STARTED -->

## Getting Started

### Prerequisites

Make sure you have yarn, git, rust, solana-cli, and anchor installed and configured on your system.

### Installation

Clone the repo,

```shell
git clone https://github.com/mgnfy-view/lego-lend.git
```

cd into the repo, and install the necessary dependencies,

```shell
cd lego-lend
yarn install
anchor build
```

Run tests by executing,

```shell
anchor test <test-name>
```

That's it, you are good to go now!

<!-- ROADMAP -->

## Roadmap

-   [ ] Solana program development
-   [ ] Unit tests
-   [x] Write a good README.md

See the [open issues](https://github.com/mgnfy-view/lego-lend/issues) for a full list of proposed features (and known issues).

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

Distributed under the MIT License. See `LICENSE.txt` for more information.

<!-- CONTACT -->

## Reach Out

Here's a gateway to all my socials, don't forget to hit me up!

[![Linktree](https://img.shields.io/badge/linktree-1de9b6?style=for-the-badge&logo=linktree&logoColor=white)][linktree-url]

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/mgnfy-view/lego-lend.svg?style=for-the-badge
[contributors-url]: https://github.com/mgnfy-view/lego-lend/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/mgnfy-view/lego-lend.svg?style=for-the-badge
[forks-url]: https://github.com/mgnfy-view/lego-lend/network/members
[stars-shield]: https://img.shields.io/github/stars/mgnfy-view/lego-lend.svg?style=for-the-badge
[stars-url]: https://github.com/mgnfy-view/lego-lend/stargazers
[issues-shield]: https://img.shields.io/github/issues/mgnfy-view/lego-lend.svg?style=for-the-badge
[issues-url]: https://github.com/mgnfy-view/lego-lend/issues
[license-shield]: https://img.shields.io/github/license/mgnfy-view/lego-lend.svg?style=for-the-badge
[license-url]: https://github.com/mgnfy-view/lego-lend/blob/master/LICENSE.txt
[linktree-url]: https://linktr.ee/mgnfy.view
