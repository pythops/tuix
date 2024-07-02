<div align="center">
  <h1> 🚧⚒️ Work In Progress ⚒️🚧 </h1>
  <br>
  <img height="100" src="assets/logo.png"/>
  <h2> TUI for managing screens </h2>
  <img src="https://github.com/pythops/tuix/assets/57548585/cc822af3-a854-4834-aaf9-834bfc05f7ed"/>
</div>

> [!NOTE]
> Only two monitors (one primary and one secondary) are supported at this point.

## 💡Prerequisites

You need to install [xrandr](https://command-not-found.com/xrandr)

## 🚀 Run

Run the following command:

```shell
git clone https://github.com/pythops/tuix
cd tuix
cargo run
```

## 🪄 Usage

`h` or `left`: Move left.

`j` or `Down`: Move below.

`k` or `Up` : Move above.

`l` or `right` : Move right.

`Enter`: Apply the changes.

`Esc`: Dismiss the move.

`?`: Show help popup

## 📝 Todo

- [ ] Enable/Disable monitors.
- [ ] Save/Restore layout autorandr style.
- [ ] Handle multiple external monitors.
- [ ] Support wayland

## ⚖️ License

GPLv3
