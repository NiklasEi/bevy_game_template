#!/usr/bin/env sh

rm -rf AppIcon.iconset/*
mkdir -p AppIcon.iconset
sips -z 16 16     icon_1024x1024.png --out AppIcon.iconset/icon_16x16.png
sips -z 32 32     icon_1024x1024.png --out AppIcon.iconset/icon_16x16@2x.png
sips -z 32 32     icon_1024x1024.png --out AppIcon.iconset/icon_32x32.png
sips -z 64 64     icon_1024x1024.png --out AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128   icon_1024x1024.png --out AppIcon.iconset/icon_128x128.png
sips -z 256 256   icon_1024x1024.png --out AppIcon.iconset/icon_128x128@2x.png
sips -z 256 256   icon_1024x1024.png --out AppIcon.iconset/icon_256x256.png
sips -z 512 512   icon_1024x1024.png --out AppIcon.iconset/icon_256x256@2x.png
sips -z 512 512   icon_1024x1024.png --out AppIcon.iconset/icon_512x512.png
cp icon_1024x1024.png AppIcon.iconset/icon_512x512@2x.png
iconutil -c icns AppIcon.iconset
mkdir -p src/Game.app/Contents/Resources
mv AppIcon.icns src/Game.app/Contents/Resources/
