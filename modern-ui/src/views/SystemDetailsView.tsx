import React, { useState, useEffect } from 'react';
import { useTheme } from '../context/ThemeContext';
import { LiveTelemetry, SystemHardware, ZramConfig } from '../types';
import { fetchSystemHardware, fetchLiveTelemetry, fetchZramStatus, EMPTY_HARDWARE, EMPTY_TELEMETRY } from '../services/tauriBridge';
import { M3Card } from '../components/m3/M3Card';
import { M3Gauge } from '../components/m3/M3Gauge';
import { 
  ArrowLeft, 
  Cpu, 
  HardDrive, 
  Layers, 
  Activity, 
  Thermometer, 
  Clock, 
  Server, 
  Zap,
  RotateCcw,
  Sparkles,
  Laptop
} from 'lucide-react';
import { motion } from 'motion/react';

interface SystemDetailsViewProps {
  onBack: () => void;
  onOpenTerminalAction: (title: string, cmd: string, steps: string[]) => void;
}

export const SystemDetailsView: React.FC<SystemDetailsViewProps> = ({ onBack, onOpenTerminalAction }) => {
  const { colors, distro, isDark } = useTheme();
  const [hw, setHw] = useState<SystemHardware>(EMPTY_HARDWARE);
  const [telemetry, setTelemetry] = useState<LiveTelemetry>(EMPTY_TELEMETRY);
  const [zram, setZram] = useState<ZramConfig | null>(null);

  useEffect(() => {
    const loadHardware = () => {
      void fetchSystemHardware(distro).then(setHw);
      void fetchZramStatus().then(setZram);
    };
    loadHardware();
    window.addEventListener('blazehouse:data-refreshed', loadHardware);
    return () => window.removeEventListener('blazehouse:data-refreshed', loadHardware);
  }, [distro]);

  // Live telemetry polling from Rust backend
  useEffect(() => {
    const updateTelemetry = async (force = false) => {
      try {
        const live = await fetchLiveTelemetry(force);
        if (live && live.cpuUsagePct !== undefined) {
          setTelemetry(live);
        }
      } catch (e) {
        console.error('Failed to fetch telemetry:', e);
      }
    };

    updateTelemetry();
    const interval = setInterval(() => { void updateTelemetry(true); }, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex flex-col gap-6 w-full max-w-6xl mx-auto p-2 sm:p-4">
      {/* 🧭 Top Subview Header (Image 3 inspired) */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <button
            id="back-to-home-btn"
            onClick={onBack}
            className="flex items-center gap-2 px-4 py-2 rounded-full text-xs font-bold transition-all hover:scale-105 active:scale-95 shadow-sm border"
            style={{
              backgroundColor: colors.surfaceContainerHigh,
              borderColor: colors.outlineVariant + '44',
              color: isDark ? '#FFF' : '#111',
            }}
          >
            <ArrowLeft className="w-4 h-4" />
            <span>Geri (Back)</span>
          </button>
          <div>
            <h2 className="text-xl sm:text-2xl font-extrabold tracking-tight" style={{ color: isDark ? '#FFF' : '#111' }}>
              System details
            </h2>
            <p className="text-xs opacity-60">
              Live status and all hardware components.
            </p>
          </div>
        </div>

        <button
          onClick={() =>
            onOpenTerminalAction(
              'Canlı Donanım ve Sensör Raporu',
              'Blaze House yerel telemetri sorgusu',
              [
                `System: ${hw.distroName} ${hw.distroVersion} • Kernel ${hw.kernelVersion}`,
                `Desktop: ${hw.desktopEnvironment} (${hw.displayServer})`,
                `Machine: ${hw.vendor} ${hw.modelName}`,
                `CPU: ${hw.cpuName} • ${hw.cpuCores} çekirdek / ${hw.cpuThreads} izlek`,
                `Graphics: ${hw.gpuName}`,
                `Sensors: CPU ${telemetry.cpuTempC > 0 ? `${telemetry.cpuTempC.toFixed(1)}°C` : 'okunamadı'} • GPU ${telemetry.gpuTempC > 0 ? `${telemetry.gpuTempC.toFixed(1)}°C` : 'okunamadı'} • Fan ${telemetry.fanRpm > 0 ? `${telemetry.fanRpm} RPM` : 'okunamadı'}`,
              ]
            )
          }
          className="px-4 py-2 rounded-full text-xs font-bold transition-all hover:scale-105 active:scale-95 shadow-sm border flex items-center gap-1.5"
          style={{
            backgroundColor: colors.primaryContainer,
            borderColor: colors.primary + '44',
            color: colors.onPrimaryContainer,
          }}
        >
          <Sparkles className="w-3.5 h-3.5" />
          <span>Canlı Sistem Raporu</span>
        </button>
      </div>

      {/* 📊 LIVE STATUS CARD (Circular Gauges for CPU, RAM, Disk + Live Stats) */}
      <M3Card variant="elevated" className="p-6 sm:p-8">
        <div className="flex items-center gap-2 mb-6">
          <div
            className="w-8 h-8 rounded-xl flex items-center justify-center"
            style={{ backgroundColor: colors.primaryContainer, color: colors.onPrimaryContainer }}
          >
            <Activity className="w-4 h-4" />
          </div>
          <div>
            <h3 className="text-base font-bold tracking-tight">Live status (Canlı Durum)</h3>
            <p className="text-xs opacity-60">Gerçek zamanlı kaynak tüketimi ve sensör verileri</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-center">
          {/* Circular Gauges Row */}
          <div className="lg:col-span-8 flex flex-wrap items-center justify-around gap-6">
            <M3Gauge
              id="gauge-cpu"
              value={telemetry?.cpuUsagePct ?? 0}
              title="CPU"
              subtitle={`${hw?.cpuCores ?? 8} Çekirdek • ${hw?.cpuThreads ?? 16} İş Parçacığı`}
              color={colors.primary}
              size={135}
            />

            <M3Gauge
              id="gauge-ram"
              value={telemetry?.ramUsagePct ?? 0}
              title="RAM"
              subtitle={`${telemetry?.ramUsageGiB ?? 0} / ${hw?.totalRamGiB ?? 0} GiB`}
              color={colors.secondary}
              size={135}
            />

            <M3Gauge
              id="gauge-disk"
              value={telemetry?.diskUsagePct ?? 0}
              title="Disk"
              subtitle={`${telemetry?.diskUsageGiB ?? 0} / ${hw?.diskTotalGiB ?? 0} GiB`}
              color={colors.tertiary}
              size={135}
            />
          </div>

          {/* Quick Metrics Column (Temperature, Swap, Uptime) */}
          <div
            className="lg:col-span-4 flex flex-col gap-3.5 p-5 rounded-2xl border shadow-sm"
            style={{
              backgroundColor: colors.surfaceContainerHigh,
              borderColor: colors.outlineVariant + '44',
            }}
          >
            <div className="flex items-center justify-between pb-2 border-b border-white/5">
              <div className="flex items-center gap-2">
                <Thermometer className="w-4 h-4 text-amber-400" />
                <span className="text-xs font-semibold">Temperature (Sıcaklık)</span>
              </div>
              <span className="text-xs font-mono font-bold text-amber-400">
                {telemetry.cpuTempC > 0 ? `${telemetry.cpuTempC.toFixed(1)} °C` : 'Okunamadı'}
              </span>
            </div>

            <div className="flex items-center justify-between pb-2 border-b border-white/5">
              <div className="flex items-center gap-2">
                <Layers className="w-4 h-4 text-blue-400" />
                <span className="text-xs font-semibold">Swap (Takas Alanı)</span>
              </div>
              <span className="text-xs font-mono font-bold">
                {(telemetry?.swapUsageGiB ?? 0).toFixed(1)} / {hw?.totalSwapGiB ?? 0} GiB
              </span>
            </div>

            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Clock className="w-4 h-4 text-emerald-400" />
                <span className="text-xs font-semibold">Uptime (Çalışma Süresi)</span>
              </div>
              <span className="text-xs font-mono font-bold text-emerald-400">
                {hw?.uptime ?? 'Aktif'}
              </span>
            </div>
          </div>
        </div>
      </M3Card>

      {/* 💻 PROCESSOR & MEMORY CARDS (Side by Side matching Image 3) */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Processor Card */}
        <M3Card variant="filled" className="flex flex-col justify-between">
          <div>
            <div className="flex items-center gap-3 mb-5">
              <div
                className="w-9 h-9 rounded-2xl flex items-center justify-center"
                style={{ backgroundColor: colors.primaryContainer, color: colors.onPrimaryContainer }}
              >
                <Cpu className="w-5 h-5" />
              </div>
              <div>
                <h3 className="text-base font-bold">Processor (İşlemci)</h3>
                <p className="text-xs opacity-60">Frekanslar ve çekirdek yük dağılımı</p>
              </div>
            </div>

            <div className="space-y-3 text-xs">
              <div className="flex flex-col gap-1 pb-2 border-b border-white/5">
                <span className="opacity-60 text-[11px] uppercase font-bold">İşlemci Modeli</span>
                <span className="font-mono font-bold text-sm" style={{ color: isDark ? '#FFF' : '#111' }}>
                  {hw.cpuName}
                </span>
              </div>

              <div className="grid grid-cols-2 gap-4 py-2 border-b border-white/5">
                <div>
                  <span className="opacity-60 text-[11px] block">Çekirdek / İş Parçacığı</span>
                  <span className="font-mono font-bold text-sm">{hw.cpuCores} / {hw.cpuThreads}</span>
                </div>
                <div>
                  <span className="opacity-60 text-[11px] block">Minimum Frekans</span>
                  <span className="font-mono font-bold text-sm">{hw.baseClockGhz.toFixed(1)} GHz</span>
                </div>
              </div>

              <div className="flex items-center justify-between py-1">
                <span className="opacity-60 text-[11px]">Maksimum Boost Frekans</span>
                <span className="font-mono font-bold text-sm text-amber-400">{hw.maxClockGhz.toFixed(1)} GHz</span>
              </div>
            </div>
          </div>

          {/* Per-Core Live Frequency Bars */}
          <div className="mt-6 pt-4 border-t border-white/5">
            <div className="flex items-center justify-between mb-2">
              <span className="text-[11px] font-bold opacity-70">Çekirdek Frekans Dağılımı</span>
              <span className="text-[10px] font-mono opacity-50">
                Ortalama: {telemetry.coreClocksMhz.length > 0 ? (telemetry.coreClocksMhz.reduce((sum, mhz) => sum + mhz, 0) / telemetry.coreClocksMhz.length / 1000).toFixed(2) : '0.00'} GHz
              </span>
            </div>
            <div className="grid grid-cols-4 sm:grid-cols-8 gap-1.5">
              {(telemetry?.coreClocksMhz || []).map((mhz, idx) => (
                <div key={idx} className="flex flex-col items-center gap-1">
                  <div className="w-full h-12 bg-white/5 rounded-md relative overflow-hidden flex items-end p-0.5">
                    <motion.div
                      className="w-full rounded-sm"
                      style={{
                        backgroundColor: colors.primary,
                        height: `${Math.max(10, Math.min(100, ((mhz - 1000) / 4500) * 100))}%`,
                      }}
                      transition={{ duration: 0.3 }}
                    />
                  </div>
                  <span className="text-[9px] font-mono opacity-60">C{idx}</span>
                </div>
              ))}
            </div>
          </div>
        </M3Card>

        {/* Memory Card */}
        <M3Card variant="filled" className="flex flex-col justify-between">
          <div>
            <div className="flex items-center gap-3 mb-5">
              <div
                className="w-9 h-9 rounded-2xl flex items-center justify-center"
                style={{ backgroundColor: colors.secondaryContainer, color: colors.onSecondaryContainer }}
              >
                <Layers className="w-5 h-5" />
              </div>
              <div>
                <h3 className="text-base font-bold">Memory (Bellek & RAM)</h3>
                <p className="text-xs opacity-60">Fiziksel RAM, önbellek ve kullanılabilir bellek</p>
              </div>
            </div>

            <div className="space-y-3.5 text-xs">
              <div className="flex items-center justify-between pb-2 border-b border-white/5">
                <span className="opacity-70 font-medium">Toplam Fiziksel RAM</span>
                <span className="font-mono font-bold text-sm text-emerald-400">
                  {hw.totalRamGiB.toFixed(1)} GiB
                </span>
              </div>

              <div className="flex items-center justify-between pb-2 border-b border-white/5">
                <span className="opacity-70 font-medium">Toplam Takas Alanı (Swap)</span>
                <span className="font-mono font-bold text-sm">
                  {hw.totalSwapGiB.toFixed(1)} GiB
                </span>
              </div>

              <div className="flex items-center justify-between pb-2 border-b border-white/5">
                <span className="opacity-70 font-medium">ZRAM Sıkıştırılmış Alan</span>
                <span className="font-mono font-bold text-sm text-amber-400">
                  {zram ? (zram.enabled ? `${zram.allocatedSizeGiB.toFixed(1)} GiB (${zram.algorithm})` : 'Devre dışı') : 'Okunuyor…'}
                </span>
              </div>

              <div className="flex items-center justify-between py-1">
                <span className="opacity-70 font-medium">Önbellek & Buffer (Cache)</span>
                <span className="font-mono font-bold text-sm">
                  {telemetry.ramCacheGiB.toFixed(1)} GiB
                </span>
              </div>
            </div>
          </div>

          {/* Memory Visual Stack */}
          <div className="mt-6 pt-4 border-t border-white/5">
            <div className="flex items-center justify-between text-[11px] mb-2">
              <span className="font-bold opacity-70">RAM Kullanım Dağılımı</span>
              <span className="font-mono text-emerald-400">{telemetry.ramAvailableGiB.toFixed(1)} GiB Kullanılabilir</span>
            </div>
            <div className="w-full h-3 rounded-full bg-white/5 overflow-hidden flex gap-0.5">
              <div className="h-full bg-amber-500 rounded-l-full" style={{ width: `${telemetry.ramUsagePct}%` }} title={`Kullanım (${telemetry.ramUsageGiB.toFixed(1)} GiB)`} />
              <div className="h-full bg-emerald-500/30 flex-1 rounded-r-full" title="Kullanılabilir Boş RAM" />
            </div>
            <div className="flex items-center gap-4 text-[10px] mt-2 opacity-60">
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-amber-500" /> Aktif</span>
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-emerald-500/40" /> Boş</span>
            </div>
          </div>
        </M3Card>
      </div>

      {/* 🐧 SYSTEM INFORMATION & DISTRO SPECS */}
      <M3Card variant="outlined" className="p-6">
        <div className="flex items-center gap-3 mb-4">
          <div
            className="w-9 h-9 rounded-2xl flex items-center justify-center"
            style={{ backgroundColor: colors.tertiaryContainer, color: isDark ? '#FFF' : '#111' }}
          >
            <Server className="w-5 h-5" />
          </div>
          <div>
            <h3 className="text-base font-bold">İşletim Sistemi & Çekirdek (Kernel) Detayları</h3>
            <p className="text-xs opacity-60">Çalışan sistemden doğrudan okunan bilgiler</p>
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 text-xs">
          <div className="p-3.5 rounded-2xl bg-white/5 flex flex-col gap-1">
            <span className="text-[11px] opacity-60">Dağıtım Versiyonu</span>
            <span className="font-bold text-sm font-mono">{hw.distroName}</span>
            <span className="text-[10px] opacity-50">{hw.distroCodename}</span>
          </div>

          <div className="p-3.5 rounded-2xl bg-white/5 flex flex-col gap-1">
            <span className="text-[11px] opacity-60">Linux Kernel</span>
            <span className="font-bold text-sm font-mono text-emerald-400">{hw.kernelVersion}</span>
            <span className="text-[10px] opacity-50">Çalışan çekirdek sürümü</span>
          </div>

          <div className="p-3.5 rounded-2xl bg-white/5 flex flex-col gap-1">
            <span className="text-[11px] opacity-60">Masaüstü Ortamı</span>
            <span className="font-bold text-sm">
              {hw.desktopEnvironment && !hw.desktopEnvironment.toLowerCase().includes("niri") ? hw.desktopEnvironment : "SolarUI"}
            </span>
            <span className="text-[10px] opacity-50">XDG masaüstü oturumu</span>
          </div>

          <div className="p-3.5 rounded-2xl bg-white/5 flex flex-col gap-1">
            <span className="text-[11px] opacity-60">Görüntü Protokolü</span>
            <span className="font-bold text-sm">{hw.displayServer}</span>
            <span className="text-[10px] opacity-50">Oturum görüntü protokolü</span>
          </div>
        </div>
      </M3Card>
    </div>
  );
};
