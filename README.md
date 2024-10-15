# Typora image to base64

Automatically convert images in the HTML output from Typora to base64.

## Support Types

- [x] Local image(Absolute/Relative Path)
- [x] Online image

## Supported Image Types

- [x] JPEG
- [x] PNG
- [x] GIF
- [x] BMP
- [x] TIFF
- [x] ICO
- [x] WebP

## Installation

### Install from crates.io

```bash
cargo install typora-img-to-base64
```

or

### Install from Source

```bash
git clone https://github.com/hayd1n/typora-img-to-base64
cd typora-img-to-base64
cargo install --path .
```

## Usage

```bash
typora-img-to-base64 <currentPath> <outputPath>
```

## Using in Typora

Set the following custom command in Typora's export setting

```
typora-img-to-base64 "${currentPath}" "${outputPath}"
```

<img width="832" alt="Screenshot 2024-10-16 at 2 56 30 AM" src="https://github.com/user-attachments/assets/408ea0c3-412d-4590-bd34-4206c1546220">
