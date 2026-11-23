# Blaze House — Sistem Gereksinimleri

Bu belge, Blaze House'un açılıp çalışması için gereken şartları listeler.

Blaze House **gömülü venv** ile dağıtılır — yani PyQt6 ve Python bağımlılıkları
paketin içindedir. Sistemde ayrıca Python paketi kurmana **gerek yoktur**. Aşağıdaki
gereksinimler sistem/grafik yığınına ait, venv'e gömülemeyen bileşenlerdir.

---

## Zorunlu (uygulama bunlarsız açılmaz)

| Bileşen | Neden gerekli | Debian/Ubuntu paketi | Fedora paketi |
|---|---|---|---|
| glibc 2.31+ | Temel çalışma zamanı | (sistemde var) | (sistemde var) |
| Qt6 platform eklentisi (xcb) | Pencereyi X11'de çizmek | `libxcb-cursor0`, `libxkbcommon-x11-0` | `xcb-util-cursor`, `libxkbcommon-x11` |
| Temel X11/Wayland kütüphaneleri | Grafik yüzeyi | `libxcb1`, `libxkbcommon0` | `libxcb`, `libxkbcommon` |
| Çalışan masaüstü oturumu | GUI göstermek | X11 veya Wayland oturumu | X11 veya Wayland oturumu |

> Qt platform eklentileri (xcb/wayland) sistem grafik yığınına bağlı olduğundan
> venv'e gömülemez; bu yüzden zorunlu sistem bağımlılığıdır. Modern bir masaüstü
> kurulumunda (GNOME/KDE/vb.) bunlar zaten kuruludur.

**Hızlı kurulum (eksikse):**
```bash
# Debian/Ubuntu
sudo apt install libxcb-cursor0 libxkbcommon-x11-0

# Fedora
sudo dnf install xcb-util-cursor libxkbcommon-x11
```

---

## Opsiyonel (yoksa ilgili özellik "kullanılamıyor" olur, uygulama yine açılır)

Blaze House her sistem aracını çalışma anında yoklar (`shutil.which`). Yoksa o
özelliği gizler veya devre dışı bırakır — **uygulama çökmemez**.

| Araç | Hangi özellik için | Yoksa ne olur |
|---|---|---|
| `pkexec` (Polkit) | Yetki gerektiren tüm işlemler (servis, sysctl, ZRAM, temizlik) | Ayrıcalıklı işlemler çalışmaz; okuma/görüntüleme çalışır |
| `gsettings` (GNOME) | Gizlilik & performans tweak'leri, animasyon kapatma | Bu tweak'ler "kullanılamıyor" |
| `systemctl` (systemd) | Servis aç/kapat (Bluetooth, CUPS, Avahi) | Servis kartları gizli |
| `apt` / `apt-get` veya `dnf` | Paket önbelleği temizleme, yetim paket kaldırma | İlgili temizlik eylemleri çalışmaz |
| `journalctl` (systemd) | Sistem günlüğü küçültme | O temizlik eylemi çalışmaz |
| `sysctl` | swappiness / vfs_cache ayarı | Bu performans tweak'leri çalışmaz |
| `powerprofilesctl` (power-profiles-daemon) | Güç profili değiştirme (Pil sekmesi) | Güç profili kartı "bulunamadı" |
| `xdg-mime` | Gerçek varsayılan uygulama tespiti (Uygulamalar) | Varsayılanlar "ayarlı değil" görünür |
| `lsblk`, `lspci` | Donanım envanteri (Sistem detayları) | Bazı donanım satırları boş kalır |
| `zramctl` / `/sys/block/zram0` | ZRAM durumu ve yapılandırma | ZRAM sayfası "kullanılamıyor" |

**BlazeOS için öneri:** Tam deneyim için ISO'ya şunları dahil et:
`polkit` (pkexec), `systemd`, `power-profiles-daemon`, `util-linux` (lsblk/zramctl),
`pciutils` (lspci), `xdg-utils` (xdg-mime). GNOME tabanlıysa `gsettings` zaten gelir.

---

## Donanım / ortam

- **Mimari:** x86_64 (paketler bu mimari için üretildi)
- **Disk:** ~250 MB (gömülü venv + PyQt6 dahil kurulu boyut)
- **RAM:** minimum ~200 MB boş (Qt uygulaması); edition tespiti için önemi yok
- **Ekran:** grafik oturumu zorunlu (saf TTY'de açılmaz — bu bir GUI uygulaması)

---

## Özet: en yalın çalışma şartı

Modern bir masaüstü Linux'ta (GNOME/KDE/XFCE, X11 veya Wayland) Blaze House
**ek bir şey kurmadan açılır**. Sadece çok minimal/başsız sistemlerde yukarıdaki
zorunlu Qt xcb kütüphaneleri eksik olabilir. Opsiyonel araçlar arttıkça daha
fazla özellik aktif olur; hiçbiri yoksa bile uygulama açılır ve donanım/sistem
bilgilerini görüntüler.
