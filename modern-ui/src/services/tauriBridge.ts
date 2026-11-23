import { DistroBase, LiveTelemetry, PackageUpdate, RealStorageDetails, SystemHardware, UpdateChannelInfo, ZramConfig } from '../types';

export const isTauri = (): boolean => {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
};

export const EMPTY_HARDWARE: SystemHardware = {
  modelName: 'Masaüstü uygulamasında algılanacak', vendor: 'Tarayıcı önizlemesi', cpuName: 'Donanım verisi yok',
  cpuCores: 0, cpuThreads: 0, baseClockGhz: 0, maxClockGhz: 0, gpuName: 'Donanım verisi yok', gpuVramGiB: 0,
  totalRamGiB: 0, totalSwapGiB: 0, diskTotalGiB: 0, distroName: 'Tarayıcı Önizlemesi', distroId: 'fedora',
  distroVersion: '', distroCodename: '', kernelVersion: 'Masaüstü sürümünde okunur', desktopEnvironment: 'Web',
  displayServer: 'Browser', uptime: '—', chassisType: 'desktop',
};

export const EMPTY_TELEMETRY: LiveTelemetry = { cpuUsagePct: 0, ramUsageGiB: 0, ramUsagePct: 0, ramAvailableGiB: 0, ramCacheGiB: 0, diskUsageGiB: 0, diskUsagePct: 0, swapUsageGiB: 0, swapUsagePct: 0, cpuTempC: 0, gpuTempC: 0, fanRpm: 0, coreClocksMhz: [], networkDownKbps: 0, networkUpKbps: 0 };

// Lazy load Tauri API when running inside Tauri
const getTauriInvoke = async () => {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke;
  }
  return null;
};

const desktopOnly = (): never => {
  throw new Error('Bu işlem yalnızca kurulu Blaze House masaüstü uygulamasında kullanılabilir. Tarayıcı önizlemesi sistem ayarlarını değiştirmez.');
};

let hardwareCache: Promise<SystemHardware> | null = null;
let telemetryCache: Promise<LiveTelemetry> | null = null;
let zramCache: Promise<ZramConfig> | null = null;
let storageCache: Promise<RealStorageDetails> | null = null;
const updatesCache = new Map<DistroBase, Promise<PackageUpdate[]>>();

// Window Controls
export const closeWindow = async (): Promise<void> => {
  if (isTauri()) {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    await getCurrentWindow().close();
  }
};

export const minimizeWindow = async (): Promise<void> => {
  if (isTauri()) {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    await getCurrentWindow().minimize();
  }
};

export const toggleMaximizeWindow = async (): Promise<void> => {
  if (isTauri()) {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    await getCurrentWindow().toggleMaximize();
  } else {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => {});
    } else {
      document.exitFullscreen().catch(() => {});
    }
  }
};

const HARDWARE_STORAGE_KEY = 'blazehouse:hardware_cache';

// System Hardware
export const fetchSystemHardware = (distro: DistroBase, force = false): Promise<SystemHardware> => {
  if (force) {
    hardwareCache = null;
    try {
      localStorage.removeItem(HARDWARE_STORAGE_KEY);
    } catch {}
  } else if (hardwareCache) {
    return hardwareCache;
  } else {
    try {
      const cached = localStorage.getItem(HARDWARE_STORAGE_KEY);
      if (cached) {
        const parsed = JSON.parse(cached) as SystemHardware;
        hardwareCache = Promise.resolve(parsed);
        // Refresh silently in background without blocking UI
        setTimeout(() => {
          void fetchSystemHardware(distro, true);
        }, 300);
        return hardwareCache;
      }
    } catch {}
  }

  hardwareCache = (async () => {
    try {
      const invoke = await getTauriInvoke();
      if (invoke) {
        const cmd = force ? 'refresh_system_hardware' : 'get_system_hardware';
        const data = await invoke<SystemHardware>(cmd).catch(() => invoke<SystemHardware>('get_system_hardware'));
        if (data && data.cpuName) {
          try {
            localStorage.setItem(HARDWARE_STORAGE_KEY, JSON.stringify(data));
          } catch {}
          return data;
        }
      }
    } catch (err) {
      console.warn('Tauri get_system_hardware failed:', err);
    }
    return { ...EMPTY_HARDWARE, distroId: distro };
  })();
  return hardwareCache;
};

// Live Telemetry
export const fetchLiveTelemetry = (force = false): Promise<LiveTelemetry> => {
  if (!force && telemetryCache) return telemetryCache;
  telemetryCache = (async () => {
    try {
      const invoke = await getTauriInvoke();
      if (invoke) return await invoke<LiveTelemetry>('get_live_telemetry');
    } catch (err) {
      console.warn('Tauri get_live_telemetry failed:', err);
    }
    return EMPTY_TELEMETRY;
  })();
  return telemetryCache;
};

// ZRAM
export const fetchZramStatus = (force = false): Promise<ZramConfig> => {
  if (!force && zramCache) return zramCache;
  zramCache = (async () => {
    try {
      const invoke = await getTauriInvoke();
      if (invoke) return await invoke<ZramConfig>('get_zram_status');
    } catch (err) {
      console.warn('Tauri get_zram_status failed:', err);
      zramCache = null;
      throw err;
    }
    return { enabled: false, allocatedSizeGiB: 0, algorithm: 'zstd', compressionRatio: 1, swappiness: 60, priority: 100, actualUsedMiB: 0, compressedUsedMiB: 0 };
  })();
  return zramCache;
};

export const applyZramConfigToSystem = async (
  enabled: boolean,
  allocatedSizeGiB: number,
  algorithm: string,
  swappiness: number,
  priority: number
): Promise<string[]> => {
  try {
    const invoke = await getTauriInvoke();
    if (invoke) {
      const result = await invoke<string[]>('apply_zram_config', {
        enabled,
        allocatedSizeGib: allocatedSizeGiB,
        algorithm,
        swappiness,
        priority,
      });
      zramCache = null;
      return result;
    }
  } catch (err) {
    console.warn('Tauri apply_zram_config failed:', err);
    throw err;
  }
  return desktopOnly();
};

// System Updates
export const fetchSystemUpdates = (distro: DistroBase, force = false): Promise<PackageUpdate[]> => {
  if (!force && updatesCache.has(distro)) return updatesCache.get(distro)!;
  const request = (async () => {
    try {
      const invoke = await getTauriInvoke();
      if (invoke) return (await invoke<PackageUpdate[]>('check_system_updates', { distro })) || [];
    } catch (err) {
      updatesCache.delete(distro);
      throw err;
    }
    throw new Error('Paket sorgusu masaüstü uygulamasında kullanılabilir.');
  })();
  updatesCache.set(distro, request);
  return request;
};

// Storage Details
export const fetchStorageDetails = (force = false): Promise<RealStorageDetails> => {
  if (!force && storageCache) return storageCache;
  storageCache = (async () => {
    try {
      const invoke = await getTauriInvoke();
      if (invoke) return await invoke<RealStorageDetails>('get_storage_details');
    } catch (err) {
      console.warn('Tauri get_storage_details failed:', err);
      storageCache = null;
      throw err;
    }
    return {
      diskName: 'Masaüstü uygulamasında algılanacak',
      mountPoint: '/',
      totalDiskGb: 0, usedDiskGb: 0, freeDiskGb: 0, usedPct: 0, categories: [],
    };
  })();
  return storageCache;
};

// Storage Cleaning
export const cleanSystemStorage = async (categoryIds: string[]): Promise<string[]> => {
  try {
    const invoke = await getTauriInvoke();
    if (invoke) {
      const result = await invoke<string[]>('clean_storage', { categoryIds });
      storageCache = null;
      return result;
    }
  } catch (err) {
    console.warn('Tauri clean_storage failed:', err);
    throw err;
  }
  return desktopOnly();
};

export const preloadSystemData = async (distro: DistroBase, force = false): Promise<void> => {
  if (force) {
    telemetryCache = null;
    zramCache = null;
    storageCache = null;
    updatesCache.clear();
  }
  const tasks: Promise<unknown>[] = [
    fetchSystemHardware(distro, force),
    fetchLiveTelemetry(force),
    fetchZramStatus(force),
    fetchStorageDetails(force),
  ];
  if (force) {
    tasks.push(fetchSystemUpdates(distro, force));
  }
  await Promise.allSettled(tasks);
};

// Generic Privileged Command Runner
export const executePrivilegedCommand = async (command: string): Promise<string[]> => {
  try {
    const invoke = await getTauriInvoke();
    if (invoke) {
      return await invoke<string[]>('run_privileged_command', { command });
    }
  } catch (err) {
    console.warn('Tauri run_privileged_command failed:', err);
    throw err;
  }
  return desktopOnly();
};

export interface OfflineUpdateProgress {
  phase: 'starting' | 'metadata' | 'downloading' | 'verifying' | 'transaction' | 'ready' | 'error';
  label: string;
  percent: number;
  current: number;
  total: number;
  package: string;
  line: string;
  done: boolean;
  error?: string | null;
}

export interface OfflineUpdateStatus {
  prepared: boolean;
  stateSummary: string;
}

export const subscribeOfflineUpdateProgress = async (
  onProgress: (ev: OfflineUpdateProgress) => void
): Promise<() => void> => {
  if (!isTauri()) return () => {};
  const { listen } = await import('@tauri-apps/api/event');
  return await listen<OfflineUpdateProgress>('offline-update-progress', (event) => {
    onProgress(event.payload);
  });
};

export const getOfflineUpdateStatus = async (): Promise<OfflineUpdateStatus> => {
  if (!isTauri()) return { prepared: false, stateSummary: '' };
  const invoke = await getTauriInvoke();
  if (invoke) return await invoke<OfflineUpdateStatus>('get_offline_update_status');
  return { prepared: false, stateSummary: '' };
};

export const prepareOfflineUpdate = async (): Promise<string[]> => {
  const invoke = await getTauriInvoke();
  if (invoke) return await invoke<string[]>('prepare_offline_update');
  return desktopOnly();
};

export const rebootToOfflineUpdate = async (): Promise<string[]> => {
  const invoke = await getTauriInvoke();
  if (invoke) return await invoke<string[]>('reboot_to_offline_update');
  return desktopOnly();
};

export const cancelOfflineUpdate = async (): Promise<string[]> => {
  const invoke = await getTauriInvoke();
  if (invoke) return await invoke<string[]>('cancel_offline_update');
  return desktopOnly();
};

export const getUpdateChannel = async (distro: DistroBase): Promise<UpdateChannelInfo> => {
  if (isTauri()) {
    const invoke = await getTauriInvoke();
    if (invoke) return await invoke<UpdateChannelInfo>('get_update_channel', { distro });
  }
  return {
    distroFamily: distro === 'fedora' ? 'solar-evolution' : 'universe',
    productName: distro === 'fedora' ? 'Blaze SolarEvolution' : 'Blaze Universe',
    productSeries: distro === 'fedora' ? '5.05' : 'Rolling',
    githubRepo: distro === 'fedora' ? 'DarkMorpheus-pc/Blaze-SolarEvolution' : 'DarkMorpheus-pc/Blaze-Universe-',
    channelUrl:
      distro === 'fedora'
        ? 'https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/channel.json'
        : 'https://api.github.com/repos/DarkMorpheus-pc/Blaze-Universe-/releases/latest',
    rpmRepoUrl: distro === 'fedora' ? 'https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/' : '',
    currentReleasever: distro === 'fedora' ? '45' : distro,
    onlineReachable: false,
    remoteSeries: null,
    remoteBaseFedora: null,
    remotePackages: {},
    releaseNotes: 'GitHub kanalı yalnızca kurulu Blaze House masaüstü uygulamasında sorgulanır.',
    upgradeTarget: null,
    error: null,
  };
};

export const prepareSystemUpgrade = async (targetVer: string): Promise<string[]> => {
  const invoke = await getTauriInvoke();
  if (invoke) return await invoke<string[]>('prepare_system_upgrade', { targetVer });
  return desktopOnly();
};

export const rebootToSystemUpgrade = async (): Promise<string[]> => {
  const invoke = await getTauriInvoke();
  if (invoke) return await invoke<string[]>('reboot_to_system_upgrade');
  return desktopOnly();
};
