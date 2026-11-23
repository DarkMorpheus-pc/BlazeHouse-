export type DistroBase = 'debian' | 'fedora' | 'arch';

export type M3ThemeColor = 'kineticGlass' | 'amber' | 'blue' | 'crimson' | 'emerald' | 'purple' | 'neutral';

export type ThemeMode = 'dark' | 'light' | 'oled';

export interface ThemeColors {
  id: M3ThemeColor;
  name: string;
  distroHint?: string;
  primary: string;
  primaryLight: string;
  primaryDark: string;
  onPrimary: string;
  primaryContainer: string;
  onPrimaryContainer: string;
  secondary: string;
  secondaryContainer: string;
  onSecondaryContainer: string;
  tertiary: string;
  tertiaryContainer: string;
  surface: string;
  surfaceDim: string;
  surfaceBright: string;
  surfaceContainerLowest: string;
  surfaceContainerLow: string;
  surfaceContainer: string;
  surfaceContainerHigh: string;
  surfaceContainerHighest: string;
  outline: string;
  outlineVariant: string;
  glow: string;
}

export type NavTabId =
  | 'home'
  | 'system-details'
  | 'zram'
  | 'updates'
  | 'storage'
  | 'gaming'
  | 'appearance'
  | 'power';

export interface SystemHardware {
  modelName: string;
  vendor: string;
  cpuName: string;
  cpuCores: number;
  cpuThreads: number;
  baseClockGhz: number;
  maxClockGhz: number;
  gpuName: string;
  gpuVramGiB: number;
  totalRamGiB: number;
  totalSwapGiB: number;
  diskTotalGiB: number;
  distroName: string;
  distroId?: DistroBase;
  distroVersion: string;
  distroCodename: string;
  kernelVersion: string;
  desktopEnvironment: string;
  displayServer: string; // Wayland or X11
  uptime: string;
  chassisType?: 'laptop' | 'desktop';
  batteryPct?: number;
  batteryStatus?: string;
}

export interface RealStorageDetails {
  diskName: string;
  mountPoint: string;
  totalDiskGb: number;
  usedDiskGb: number;
  freeDiskGb: number;
  usedPct: number;
  categories: StorageCategory[];
}

export interface LiveTelemetry {
  cpuUsagePct: number;
  ramUsageGiB: number;
  ramUsagePct: number;
  ramAvailableGiB: number;
  ramCacheGiB: number;
  diskUsageGiB: number;
  diskUsagePct: number;
  swapUsageGiB: number;
  swapUsagePct: number;
  cpuTempC: number;
  gpuTempC: number;
  fanRpm: number;
  coreClocksMhz: number[];
  networkDownKbps: number;
  networkUpKbps: number;
}

export type ZramAlgorithm = 'zstd' | 'lz4' | 'lzo' | 'deflate';

export interface ZramConfig {
  enabled: boolean;
  allocatedSizeGiB: number;
  algorithm: ZramAlgorithm;
  compressionRatio: number;
  swappiness: number;
  priority: number;
  actualUsedMiB: number;
  compressedUsedMiB: number;
}

export interface PackageUpdate {
  id: string;
  name: string;
  currentVersion: string;
  newVersion: string;
  sizeMb: number;
  type: 'security' | 'feature' | 'kernel' | 'flatpak';
  source: 'apt' | 'dnf' | 'flatpak' | 'pacman';
  description: string;
  changelog: string[];
}

export interface UpgradeTargetInfo {
  series: string;
  targetReleasever: string;
  ready: boolean;
  minBlazeRelease: string;
  notes: string;
}

export interface UpdateChannelInfo {
  distroFamily: string;
  productName: string;
  productSeries: string;
  githubRepo: string;
  channelUrl: string;
  rpmRepoUrl: string;
  currentReleasever: string;
  onlineReachable: boolean;
  remoteSeries?: string | null;
  remoteBaseFedora?: string | null;
  remotePackages: Record<string, string>;
  releaseNotes: string;
  upgradeTarget?: UpgradeTargetInfo | null;
  error?: string | null;
}

export interface StorageCategory {
  id: string;
  name: string;
  sizeGb: number;
  color: string;
  isCleanable?: boolean;
  itemCount?: number;
  description?: string;
}

export interface PowerProfile {
  id: 'power-saver' | 'balanced' | 'performance';
  name: string;
  description: string;
  icon: string;
  cpuGovernor: string;
  fanMode: string;
}
