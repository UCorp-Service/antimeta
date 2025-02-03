#!/bin/bash

# exiftool ve ffmpeg'yi kontrol et ve yükle

echo "[*] exiftool ve ffmpeg yükleniyor..."

# exiftool'ü yükleme (Debian tabanlı sistemler için)
if ! command -v exiftool &> /dev/null
then
    echo "[*] exiftool bulunamadı, yükleniyor..."
    sudo apt update
    sudo apt install -y libimage-exiftool-perl
else
    echo "[+] exiftool zaten yüklü."
fi

# ffmpeg'i yükleme
if ! command -v ffmpeg &> /dev/null
then
    echo "[*] ffmpeg bulunamadı, yükleniyor..."
    sudo apt update
    sudo apt install -y ffmpeg
else
    echo "[+] ffmpeg zaten yüklü."
fi

echo "[*] Gereksinimler yüklendi!"

