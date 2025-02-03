# Anti Meta
AntiMeta, fotoğraf ve video dosyalarından metadata (exif bilgisi gibi) temizleyen bir araçtır. Fotoğraflar için ExifTool ve videolar için FFmpeg kullanır. Bu araç, gizliliğinizi korumanıza yardımcı olur, çünkü metadata, dosyanın oluşturulma tarihi, kamera bilgileri ve daha fazlasını içerebilir. Fotoğraf ve videoların gizlilik riski oluşturan meta verilerini tamamen silmek için geliştirilmiş güçlü bir araçtır. Bu araç ile JPEG, PNG, MP4 gibi dosyalardan tüm izleri kolayca temizleyebilirsin.

![antimeta](https://github.com/user-attachments/assets/adf7e98c-e80c-422e-b948-970f2f841dfd)


## 🚀 Özellikler
📸 Fotoğraf Metadata Temizliği: JPEG, PNG, WEBP desteği
🎥 Video Metadata Temizliği: MP4, MOV, MKV desteği
⚡ Hızlı ve güvenli işlem
🖥️ Basit komut satırı arayüzü

## Kurulum
Anti Meta, Rust dilinde yazılmıştır. Eğer Rust yüklü değilse:
`sudo apt install rustc cargo -y  # Debian/Ubuntu için`
Veya Rust resmi komutuyla:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
Projeyi Klonla ve Derle:
`git clone https://github.com/UCorp-Service/antimeta.git`
`cd metadata_cleaner`
`chmod +x install_dependencies.sh`
`./install_dependencies.sh`
`cargo build --release`

## 🚀 Kullanım
`./target/release/antimeta --file /path/to/photo.jpg` veya `video.mp4`

### 📝 Desteklenen Formatlar
**Fotoğraflar:** JPG, JPEG, PNG, WEBP
**Videolar:** MP4, MOV, MKV

### ❗️ Uyarılar
-Bu araç dosyaların orijinal hallerini üzerine yazar. Önemli dosyalar için yedek alman tavsiye edilir.

-Metadata silindikten sonra geri getirilemez.
