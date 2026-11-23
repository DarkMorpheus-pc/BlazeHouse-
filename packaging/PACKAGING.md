# Blaze House — Paketleme Rehberi

Blaze House'u **.deb** (Debian/Ubuntu) ve **.rpm** (Fedora) olarak paketlemek için.

Yaklaşım: **gömülü venv**. Uygulama, PyQt6 dahil kendi Python sanal ortamıyla
`/opt/blaze-house/` altına kurulur. Böylece dağıtımın PyQt6 sürümünden bağımsız,
her yerde aynı şekilde çalışır. Tek dezavantajı paket boyutu (~90-100 MB).

---

## 1. Gereksinimleri kur (bir kez)

Precision 7750 / Ubuntu 26.04 makinende:

```bash
# fpm ve derleme araçları
sudo apt update
sudo apt install -y ruby ruby-dev build-essential python3-venv rpm
sudo gem install fpm

# doğrula
fpm --version
```

> `rpm` paketi, Ubuntu üzerinde .rpm üretebilmek için gerekli (fpm onu kullanır).

---

## 2. Paketle

Proje kökünden:

```bash
# hem .deb hem .rpm
./packaging/build-packages.sh

# sadece biri
./packaging/build-packages.sh deb
./packaging/build-packages.sh rpm
```

Script sırasıyla:
1. `blaze_settings/` kodunu `build/stage/opt/blaze-house/` altına kopyalar
2. Orada bir venv oluşturur, içine PyQt6 + requests kurar
3. `/usr/bin/blaze-house` launcher'ını yazar (venv python'ını çağırır)
4. `.desktop` + ikonları (hicolor) yerleştirir
5. fpm ile `dist/` altına paketleri üretir

Çıktı:
```
dist/blaze-house_2.14-1_amd64.deb
dist/blaze-house-2.14-1.x86_64.rpm
```

---

## 3. Kur ve test et

**Debian/Ubuntu:**
```bash
sudo apt install ./dist/blaze-house_2.14-1_amd64.deb
# veya
sudo dpkg -i ./dist/blaze-house_2.14-1_amd64.deb && sudo apt-get install -f
```

**Fedora:**
```bash
sudo dnf install ./dist/blaze-house-2.14-1.x86_64.rpm
```

Kurulumdan sonra:
- Uygulama menüsünde **Blaze House** görünür (System kategorisi)
- Terminalden: `blaze-house`

**Kaldırma:**
```bash
sudo apt remove blaze-house      # Debian/Ubuntu
sudo dnf remove blaze-house      # Fedora
```

---

## 4. Sürüm yükseltme

`build-packages.sh` içindeki `APP_VERSION` değişkenini güncelle (şu an `2.14`),
sonra tekrar paketle. `--iteration` paket revizyonudur (aynı sürümde ufak
düzeltme yaparsan artır).

---

## Paket içeriği (nereye ne kurulur)

```
/opt/blaze-house/
    blaze_settings/         # uygulama kodu
    venv/                   # gömülü Python + PyQt6
/usr/bin/blaze-house        # launcher (çalıştırılabilir)
/usr/share/applications/blaze-house.desktop
/usr/share/icons/hicolor/{16,32,48,64,128,256,512}x.../apps/blaze-house.png
```

---

## Sorun giderme

**"fpm: command not found"**
`sudo gem install fpm` çalıştır; gem bin yolu PATH'te değilse
`export PATH="$PATH:$(ruby -e 'print Gem.bindir')"`.

**.rpm üretilmiyor (Ubuntu'da)**
`sudo apt install rpm` eksik olabilir.

**Kurulumdan sonra açılmıyor**
Terminalden `blaze-house` çalıştırıp hatayı gör. Genelde eksik sistem
kütüphanesi olur (Qt'nin xcb/wayland platform eklentileri). Gerekirse:
```bash
sudo apt install libxcb-cursor0 libxkbcommon-x11-0    # Debian/Ubuntu
sudo dnf install xcb-util-cursor libxkbcommon-x11      # Fedora
```
Bu kütüphaneler Qt platform eklentisi için sistemde bulunmalı (venv'e gömülmez,
çünkü sistem grafik yığınına bağlılar).

**Paket çok büyük**
Gömülü venv PyQt6 yüzünden ~90-100 MB. Daha küçük paket istersen
`build-packages-system.sh` (sistem python3-pyqt6 bağımlılığı) alternatifini
kullan — ama o zaman dağıtımın PyQt6 sürümüne bağımlı olursun.
