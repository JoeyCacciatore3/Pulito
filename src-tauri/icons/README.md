# Icons

Generate PNG icons from the favicon.svg using these commands:

```bash
# Install required tools
sudo apt install inkscape imagemagick

# Generate icons from SVG
cd ..
inkscape -w 32 -h 32 ../static/favicon.svg -o 32x32.png
inkscape -w 128 -h 128 ../static/favicon.svg -o 128x128.png
inkscape -w 256 -h 256 ../static/favicon.svg -o 128x128@2x.png
inkscape -w 512 -h 512 ../static/favicon.svg -o icon.png

# For Windows ICO
convert 32x32.png 128x128.png 128x128@2x.png icon.png icon.ico

# For macOS ICNS (requires icnsutils)
png2icns icon.icns 32x32.png 128x128.png 128x128@2x.png icon.png
```

Or use online tools like:
- https://realfavicongenerator.net
- https://cloudconvert.com/svg-to-png
