<br/>
<p align="center">
  <a href="https://github.com/CinematicCow/pm-rs">
    <img src="https://i.imgur.com/XRJTJUa.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Project Manager</h3>

  <p align="center">
    A simple CLI to manage your projects locally
    <br/>
    <br/>
    <a href="https://github.com/CinematicCow/pm-rs/issues">Report Bug</a>
    .
    <a href="https://github.com/CinematicCow/pm-rs/issues">Request Feature</a>
  </p>
</p>

![Downloads](https://img.shields.io/github/downloads/CinematicCow/pm-rs/total) ![Issues](https://img.shields.io/github/issues/CinematicCow/pm-rs) ![License](https://img.shields.io/github/license/CinematicCow/pm-rs)

## Table Of Contents

- [About the Project](#about-the-project)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Authors](#authors)
- [Acknowledgements](#acknowledgements)

## About The Project

![Screen Shot](https://i.imgur.com/dcW7Tka.jpg)

I was tired of flying around in my terminal to navigate from one project to another. I used to use neovim's `startify` but that wasn't cutting it for me. As I had just started learning rust, I thought to build what I needed.

This is `pm`, a simple project manager of sorts written in rust. `pm` helps you open your frequently visited projects in one line. This is implemented for linux for now, but I will try to make it to work with windows too.

## Built With

`pm` is built with rust, the most beloved language, and it's not hard to see why. As a complete newbie to rust, it was pretty easy to get started and the community was also very helpful.

## Getting Started

To get started with `pm`, you can either download the binary from the downloads page or you can build it yourself.

### Installation

1. Download the latest release from the (releases page)[https://github.com/CinematicCow/pm-rs/releases/tag/v0.1.0].

2. Go to the folder you downloaded too. For example, it was downloaded to the Downloads folder.

```sh
cd ~/Downloads
```

3. Set the permissions for the binary to be executable

```sh
chmod +x ./pm
```

4. Copy the binary to the bin folder to be executable from anywhere

```sh
sudo cp pm /usr/local/bin/
```

## Usage

- Initialize pm
  First we need to initialize pm. This will create all the required folders and files locally, like a json-database.

  ```sh
  pm init
  ```

- Add a new project
  We can add a new project by specifying a name and the editor we want to use. For now there are 2 options for the editor, `neovim` & 'VScode'.
  Run `pm help add` to see more.

  To add a new project

  ```sh
  pm add -n "project name" -e vscode ~/Work/my-project
  ```

- List projects
  To list all projects you have

  ```sh
  pm list
  ```

- Open a project
  You can open a project you added, in your selected editor, by specifying the project name

  ```sh
  pm open "project name"
  ```

- Help
  To get help

  ```sh
  pm help
  ```

  To get help with a specific command

  ```sh
  pm help <Command-Name>

  pm help open
  ```

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE](https://github.com/CinematicCow/pm-rs/blob/main/LICENSE) for more information.

## Authors

- **CinematicCow** - _Software Engineer_ - [CinematicCow](https://github.com/CinematicCow)
