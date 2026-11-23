# Blaze House 5

Material 3 sistem kontrol merkezi, çevrimdışı güncelleme yöneticisi ve ana sürüm yükseltme aracı. React 19, Tauri v2 ve Rust kullanılarak BlazeOS dağıtım ailesi (**Blaze SolarEvolution** ve **Blaze Universe**) için geliştirilmiştir.

## Dizin Yapısı

- `modern-ui/`: React 19 + TypeScript arayüzü ve `src-tauri/` Rust sistem arka ucu (`blaze-house` ikilisi buradan derlenir).
- `src/`: Önceki native Iced/Rust referans uygulaması.
- `packaging/`: Hedef platformlar için `.deb` ve `.rpm` paketleme betikleri.
- `.github/workflows/release.yml`: Frontend, Rust birim testleri ve tag tabanlı paketleme otomasyonu.

## Çoklu Dağıtım Güncelleme Kanalı Mimarisi

Blaze House 5, `/etc/blazeos/update-channel.conf` ve `/etc/os-release` üzerinden çalıştığı dağıtım ailesini otomatik algılar:

- **Blaze SolarEvolution (`DarkMorpheus-pc/Blaze-SolarEvolution`)**: Fedora tabanında `https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/channel.json` manifestini okur. Standart paket güncellemelerini `dnf5 --refresh upgrade --offline` üzerinden, ana sürüm yükseltmelerini ise Btrfs anlık görüntüsü (`blaze-snapshot`) aldıktan sonra `/usr/libexec/blazeos-update system-upgrade-download` üzerinden yürütür ve canlı ilerleme yüzdesini (`offline-update-progress`) ekrana yansıtır.
- **Blaze Universe (`DarkMorpheus-pc/Blaze-Universe-`)**: Arch ve Debian tabanlı sistemlerde `pacman` / `apt` güncellemelerini ve GitHub Release kanalını takip eder.

## Temel Modüller

| Modül | Açıklama |
|---|---|
| **Donanım & Telemetri** | DMI donanım bilgileri, anlık CPU/RAM/GPU/fan telemetrisi (`sysinfo`) |
| **Güncellemeler & Yükseltme** | GitHub kanal manifesti, canlı ilerlemeli çevrimdışı DNF5 güncellemesi, ana sürüm yükseltme sihirbazı |
| **ZRAM & Performans** | zstd/lz4/lzo/deflate ZRAM yapılandırması, güç profilleri, oyun optimizasyonları |
| **Depolama & Görünüm** | Gerçek disk analizi, güvenli önbellek temizliği, Material 3 tema ve renk yönetimi |

## Geliştirme ve Derleme

Önkoşullar: Node.js 22+, Rust toolchain, Tauri v2 Linux sistem kütüphaneleri (`webkit2gtk-4.1`, `gtk3`, `librsvg`, `libayatana-appindicator`).

```bash
cd modern-ui
npm ci
npm run lint
npm run build
cd src-tauri
cargo test
cargo build --release
```

Üretilen ikili dosya: `modern-ui/src-tauri/target/release/blaze-house`.

Debian (`.deb`) ve Fedora (`.rpm`) paketleri üretmek için:

```bash
./packaging/build-packages.sh both
```

## Sistem Entegrasyonu

- **İkili Konum**: `/usr/bin/blaze-house`
- **Kanal Yapılandırması**: `/etc/blazeos/update-channel.conf`
- **Güncelleme Yardımcısı**: `/usr/libexec/blazeos-update`
- **Polkit Yetkilendirme**: `/usr/share/polkit-1/actions/org.blazeos.house.policy`

## Geliştirici ve Bağlantılar

- **Geliştirici**: Bulut Ars. E. (DarkMorpheus)
- **Blaze SolarEvolution (Fedora)**: [https://github.com/DarkMorpheus-pc/Blaze-SolarEvolution](https://github.com/DarkMorpheus-pc/Blaze-SolarEvolution)
- **Blaze House Kaynağı**: [https://github.com/DarkMorpheus-pc/BlazeHouse-](https://github.com/DarkMorpheus-pc/BlazeHouse-)
- **Web Sitesi**: [https://blazeos.com.tr](https://blazeos.com.tr)
