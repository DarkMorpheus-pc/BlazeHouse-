import React, { useEffect, useState } from 'react';
import { Cpu, Gauge, HardDrive, RefreshCw, Palette, BatteryCharging, Gamepad2, ChevronRight, Laptop, CheckCircle2 } from 'lucide-react';
import { NavTabId, ZramConfig } from '../types';
import { EMPTY_HARDWARE, fetchSystemHardware, fetchZramStatus, fetchStorageDetails, fetchSystemUpdates, isTauri } from '../services/tauriBridge';
import { useTheme } from '../context/ThemeContext';

const modules: { id: NavTabId; title: string; subtitle: string; description: string; action: string; icon: React.ElementType; tone: 'primary' | 'secondary' | 'tertiary' }[] = [
  { id: 'system-details', title: 'Sistem Detayları', subtitle: 'Hardware Telemetry', description: 'İşlemci yükünü, bellek kullanımını ve donanım mimarisini gerçek zamanlı inceleyin.', action: 'Detayları Aç', icon: Cpu, tone: 'primary' },
  { id: 'updates', title: 'Sistem Güncelleme', subtitle: 'Package Manager', description: 'Paket yöneticisinin bildirdiği güncellemeleri kontrol edin ve yeni sürümleri uygulayın.', action: 'Güncellemeleri Aç', icon: RefreshCw, tone: 'secondary' },
  { id: 'zram', title: 'ZRAM Ayarları', subtitle: 'Memory Compression', description: 'Bellek sıkıştırma durumunu izleyin; takas boyutunu ve algoritmasını yapılandırın.', action: 'ZRAM’ı Yönet', icon: Gauge, tone: 'tertiary' },
  { id: 'storage', title: 'Depolama ve Disk', subtitle: 'Drive Analysis', description: 'Disk kullanımını inceleyin; paket önbelleklerini ve eski sistem günlüklerini temizleyin.', action: 'Depolamayı Aç', icon: HardDrive, tone: 'primary' },
];

export function HomeView({ onNavigate }: { onNavigate: (id: NavTabId) => void; onOpenTerminalAction: (title: string, cmd: string, steps: string[]) => void }) {
  const { colors, distro, isDark } = useTheme();
  const [hardware, setHardware] = useState(EMPTY_HARDWARE);
  const [zram, setZram] = useState<ZramConfig | null>(null);
  const [updates, setUpdates] = useState<number | null>(null);
  const [free, setFree] = useState<number | null>(null);
  const [error, setError] = useState('');
  const desktop = isTauri();

  useEffect(() => {
    const load = () => {
      void fetchSystemHardware(distro).then(setHardware).catch(e => setError(String(e)));
      if (!desktop) return;
      void fetchZramStatus().then(setZram).catch(() => setZram(null));
      void fetchSystemUpdates(distro).then(data => setUpdates(data.length)).catch(() => setUpdates(null));
      void fetchStorageDetails().then(data => setFree(data.freeDiskGb)).catch(() => setFree(null));
    };
    load();
    window.addEventListener('blazehouse:data-refreshed', load);
    return () => window.removeEventListener('blazehouse:data-refreshed', load);
  }, [distro, desktop]);

  const metadata: Partial<Record<NavTabId, string>> = {
    'system-details': hardware.cpuCores > 0 ? `${hardware.cpuCores} Çekirdek · ${hardware.cpuThreads} İzlek` : 'Masaüstünde okunur',
    updates: updates === null ? 'Henüz doğrulanmadı' : `${updates} Güncelleme`,
    zram: zram ? `${zram.allocatedSizeGiB.toFixed(1)} GiB · ${zram.algorithm}` : 'Masaüstünde okunur',
    storage: free === null ? 'Masaüstünde okunur' : `${free.toFixed(1)} GiB Kullanılabilir`,
  };

  return (
    <div className="view-stack dashboard">
      <section className="dashboard-hero" style={{ backgroundColor: colors.surfaceContainerHigh + 'E6' }}>
        <div className="hero-copy">
          <div className="hero-overline">
            <span style={{ backgroundColor: colors.primaryContainer, color: colors.onPrimaryContainer }}>
              BLAZE SOLAREVOLUTION
            </span>
            <span>{desktop ? hardware.distroName : 'Masaüstü arayüz önizlemesi'}</span>
          </div>
          <h1>
            Blaze SolarEvolution <span className="hero-version" style={{ color: colors.primary }}>5.05</span>
          </h1>
          <p className="hero-hardware">
            <span>{desktop ? hardware.cpuName : 'Sistem ve donanım kontrol merkezi'}</span>
            <span className="divider">/</span>
            <span>{hardware.totalRamGiB > 0 ? `${hardware.totalRamGiB.toFixed(1)} GiB RAM` : 'RAM masaüstünde okunur'}</span>
            <span className="divider">/</span>
            <span style={{ color: colors.tertiary }}>{desktop ? hardware.kernelVersion : 'Linux'}</span>
          </p>
          <div className="hero-status">
            <span 
              className="cursor-pointer" 
              onClick={() => onNavigate('system-details')} 
              title="Sistem detaylarına git"
            >
              <CheckCircle2 size={13} />
              {desktop ? 'Yerel sistem oturumu' : 'Arayüz önizlemesi'}
            </span>
            <span 
              className="cursor-pointer" 
              onClick={() => onNavigate('zram')} 
              title="ZRAM ayarlarına git"
            >
              <Gauge size={13} />
              {zram ? (zram.enabled ? `ZRAM etkin · ${zram.algorithm}` : 'ZRAM devre dışı') : 'ZRAM henüz okunmadı'}
            </span>
          </div>
        </div>

        <div 
          className="hero-device cursor-pointer" 
          style={{ backgroundColor: colors.surfaceContainer + 'CC' }}
          onClick={() => onNavigate('system-details')}
          role="button"
          tabIndex={0}
          title="Sistem detaylarını görüntüle"
        >
          <span 
            className="hero-device-icon" 
            style={{ color: colors.primary, backgroundColor: colors.surfaceContainerLow }}
          >
            <Laptop size={24} />
          </span>
          <div>
            <span className="device-vendor">{desktop ? hardware.vendor : 'BLAZE HOUSE'}</span>
            <strong>{desktop ? hardware.modelName : 'Linux kontrol merkezi'}</strong>
            <small>{desktop ? hardware.gpuName : 'Gerçek veriler masaüstü oturumunda yüklenir'}</small>
          </div>
        </div>
      </section>

      {error && <p role="alert" className="text-red-400 text-xs px-2">{error}</p>}

      <div className="dashboard-grid">
        {modules.map(({ id, title, subtitle, description, action, icon: Icon, tone }) => {
          const container = tone === 'secondary' 
            ? colors.secondaryContainer 
            : tone === 'tertiary' 
            ? colors.tertiaryContainer 
            : colors.primaryContainer;
          const foreground = tone === 'secondary' 
            ? colors.onSecondaryContainer 
            : tone === 'tertiary' 
            ? (isDark ? '#F0F0CE' : '#222607') 
            : colors.onPrimaryContainer;

          return (
            <article 
              className="module-card cursor-pointer" 
              key={id} 
              style={{ backgroundColor: colors.surfaceContainer + 'D9' }}
              onClick={() => onNavigate(id)}
              role="button"
              tabIndex={0}
            >
              <div className="module-heading">
                <span className="module-icon" style={{ backgroundColor: container, color: foreground }}>
                  <Icon size={20} />
                </span>
                <div>
                  <h2>{title}</h2>
                  <p>{subtitle}</p>
                </div>
                {id === 'updates' && updates !== null && updates > 0 && (
                  <span className="module-badge" style={{ backgroundColor: colors.primary, color: colors.onPrimary }}>
                    {updates} yeni
                  </span>
                )}
              </div>
              <p className="module-description">{description}</p>
              <div className="module-footer">
                <button 
                  onClick={(e) => { e.stopPropagation(); onNavigate(id); }} 
                  style={{ 
                    backgroundColor: tone === 'primary' && id === 'system-details' ? colors.primary : container, 
                    color: tone === 'primary' && id === 'system-details' ? colors.onPrimary : foreground 
                  }}
                >
                  {action}
                  <ChevronRight size={14} />
                </button>
                <span>{metadata[id]}</span>
              </div>
            </article>
          );
        })}
      </div>

      <div className="dashboard-shortcuts">
        {([
          { id: 'gaming', label: 'Performans', subtitle: 'İşlemci ve oyun optimizasyonu', icon: Gamepad2 },
          { id: 'appearance', label: 'Görünüm', subtitle: 'Kinetic Glass · renk ve tema', icon: Palette },
          { id: 'power', label: 'Güç ve Batarya', subtitle: 'Profil ve enerji tasarrufu', icon: BatteryCharging }
        ] as const).map(({ id, label, subtitle, icon: Icon }) => (
          <button 
            key={id} 
            onClick={() => onNavigate(id)} 
            style={{ backgroundColor: colors.surfaceContainer + 'D9' }}
          >
            <span className="module-icon" style={{ backgroundColor: colors.surfaceContainerHighest, color: colors.primary }}>
              <Icon size={18} />
            </span>
            <span>
              <strong>{label}</strong>
              <small>{subtitle}</small>
            </span>
            <ChevronRight size={15} className="opacity-60 shrink-0" />
          </button>
        ))}
      </div>
    </div>
  );
}
