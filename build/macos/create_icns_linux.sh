#!/usr/bin/env sh 

rm -rf AppIcon.iconset/*
mkdir -p AppIcon.iconset
convert ../icon_1024x1024.png -resize 16x16    AppIcon.iconset/icon_16x16.png
convert ../icon_1024x1024.png -resize 32x32    AppIcon.iconset/icon_16x16@2x.png
convert ../icon_1024x1024.png -resize 32x32    AppIcon.iconset/icon_32x32.png
convert ../icon_1024x1024.png -resize 64x64    AppIcon.iconset/icon_32x32@2x.png
convert ../icon_1024x1024.png -resize 128x128  AppIcon.iconset/icon_128x128.png
convert ../icon_1024x1024.png -resize 256x256  AppIcon.iconset/icon_128x128@2x.png
convert ../icon_1024x1024.png -resize 256x256  AppIcon.iconset/icon_256x256.png
convert ../icon_1024x1024.png -resize 512x512  AppIcon.iconset/icon_256x256@2x.png
convert ../icon_1024x1024.png -resize 512x512  AppIcon.iconset/icon_512x512.png
cp ../icon_1024x1024.png AppIcon.iconset/icon_512x512@2x.png
png2icns ./AppIcon.icns AppIcon.iconset/icon_16x16.png AppIcon.iconset/icon_32x32.png AppIcon.iconset/icon_128x128.png AppIcon.iconset/icon_256x256.png AppIcon.iconset/icon_512x512.png
mkdir -p src/Game.app/Contents/Resources
mv AppIcon.icns src/Game.app/Contents/Resources/
