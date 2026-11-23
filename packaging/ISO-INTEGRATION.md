# Blaze House — BlazeOS ISO'suna Gömme Rehberi

Hedef: Blaze House'u BlazeOS'un **Debian** ve **Fedora** ISO'larına önceden
kurulu (preinstalled) olarak dahil etmek.

Yöntem: `.deb`/`.rpm` paketini ISO build sürecine dahil et. Kodu elle
kopyalamak yerine paket olarak kurmak, uygulamanın paket veritabanında
görünmesini, güncellenebilmesini ve temiz kaldırılabilmesini sağlar.

Paketler güncel React/Tauri arayüzünü ve Rust sistem köprüsünü içerir. Hedef
ISO'da WebKitGTK, GTK3 ve Polkit çalışma zamanı bağımlılıkları bulunmalıdır.

---

## Önce: paketleri üret

```bash
./packaging/build-packages.sh
# dist/blaze-house_5.0.0_amd64.deb
# dist/blaze-house-5.0.0-1.x86_64.rpm
```

---

## A) Debian tabanlı BlazeOS ISO'su

BlazeOS Debian ISO'sunu büyük olasılıkla `live-build`, `debootstrap` tabanlı bir
araç veya Cubic (grafik arayüz) ile üretiyorsun. Üçü için de yöntem:

### Seçenek 1 — live-build (önerilen, otomatik)

Yerel paketi live-build'in `packages.chroot` havuzuna koy:

```bash
# BlazeOS live-build proje kökünde:
mkdir -p config/packages.chroot/
cp dist/blaze-house_5.0.0_amd64.deb config/packages.chroot/

# Blaze House'u kurulacak paketler listesine ekle:
echo "blaze-house" >> config/package-lists/blazeos.list.chroot

# ISO'yu yeniden derle:
sudo lb build
```

live-build, `packages.chroot/` içindeki yerel .deb'i otomatik kurar; harici
depoya gerek yok.

### Seçenek 2 — Cubic (grafik, manuel chroot)

Cubic'in chroot terminalinde:

```bash
# .deb'i chroot'a kopyaladıktan sonra (Cubic bunu kolaylaştırır):
apt install ./blaze-house_5.0.0_amd64.deb
```

### Seçenek 3 — debootstrap / elle chroot

```bash
sudo cp dist/blaze-house_5.0.0_amd64.deb /mnt/blazeos-root/tmp/
sudo chroot /mnt/blazeos-root /bin/bash
apt install /tmp/blaze-house_5.0.0_amd64.deb
rm /tmp/blaze-house_5.0.0_amd64.deb
exit
```

### Doğrulama (ISO kurulduktan sonra)
```bash
dpkg -l | grep blaze-house      # kurulu görünmeli
which blaze-house               # /usr/bin/blaze-house
```

---

## B) Fedora tabanlı BlazeOS ISO'su

Fedora ISO'ları **Kickstart** (`.ks`) + `livemedia-creator`/`lorax` ile üretilir.

### Seçenek 1 — Kickstart'a yerel paket ekle (önerilen)

Yerel .rpm'i bir dizine koyup Kickstart'ta repo olarak tanımla:

```bash
# yerel repo dizini hazırla
mkdir -p /srv/blazeos-repo
cp dist/blaze-house-5.0.0-1.x86_64.rpm /srv/blazeos-repo/
createrepo_c /srv/blazeos-repo/     # (createrepo_c kurulu olmalı)
```

Kickstart dosyanda (`blazeos.ks`):

```kickstart
# yerel Blaze House deposunu tanımla
repo --name=blazeos-local --baseurl=file:///srv/blazeos-repo/

%packages
blaze-house
%end
```

Sonra ISO'yu derle:
```bash
sudo livemedia-creator --ks blazeos.ks --no-virt \
    --resultdir /var/lmc --project BlazeOS --make-iso
```

### Seçenek 2 — chroot / elle (mevcut root ağacına)

```bash
sudo cp dist/blaze-house-2.14-1.x86_64.rpm /mnt/blazeos-root/tmp/
sudo chroot /mnt/blazeos-root /bin/bash
dnf install /tmp/blaze-house-2.14-1.x86_64.rpm
rm /tmp/blaze-house-2.14-1.x86_64.rpm
exit
```

### Doğrulama
```bash
rpm -q blaze-house
which blaze-house
```

---

## İmzalama (opsiyonel ama önerilir)

Kullanıcılar paketi güvenle kursun diye imzala:

**Debian (.deb):**
```bash
# dpkg-sig ile
dpkg-sig --sign builder dist/blaze-house_2.14-1_amd64.deb
```

**Fedora (.rpm):**
```bash
# GPG anahtarını ~/.rpmmacros'a tanımladıktan sonra
rpm --addsign dist/blaze-house-2.14-1.x86_64.rpm
```

ISO içi kurulumlarda imza şart değil (yerel paket), ama GitHub'da yayınlarken
kullanıcı güveni için önerilir.

---

## Otomatik başlangıç / varsayılan (opsiyonel)

Blaze House'u ilk açılışta bir kez göstermek istersen (örn. hoş geldin akışı),
`/etc/xdg/autostart/blaze-house-welcome.desktop` ekleyebilirsin — ama bunu
paket içine koymak yerine BlazeOS'un kendi "ilk kurulum" akışına bırakmak daha
temiz. Blaze House normalde menüden açılan bir uygulamadır, autostart gerekmez.

---

## Sürüm akışı (ISO + GitHub tutarlılığı)

1. `build-packages.sh` içindeki `APP_VERSION`'ı güncelle
2. Paketleri üret
3. Aynı paketleri hem ISO'ya göm hem GitHub Releases'e yükle
4. Böylece ISO'daki sürüm ile GitHub'daki sürüm birebir aynı olur

Uygulama içi güncelleme (GitHub'dan sürüm kontrolü) zaten Blaze House'un
Hakkında sekmesinde mevcut (updater.py) — GitHub'da release yayınladığında
otomatik çalışır.
