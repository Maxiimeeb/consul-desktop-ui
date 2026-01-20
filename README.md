# consul-desktop-ui
Desktop UI for consul


# Dev

## Requirements
```
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

## Backend
```
pnpm run tauri dev
```

## Bundle
```
pnpm run tauri build
```