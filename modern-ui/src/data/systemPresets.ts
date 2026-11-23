import { PowerProfile } from '../types';

export const POWER_PROFILES: PowerProfile[] = [
  { id: 'performance', name: 'Yüksek Performans', description: 'power-profiles-daemon performans profili.', icon: 'Zap', cpuGovernor: 'Sistem tarafından yönetilir', fanMode: 'Donanım tarafından yönetilir' },
  { id: 'balanced', name: 'Dengeli (Önerilen)', description: 'power-profiles-daemon dengeli profili.', icon: 'Scale', cpuGovernor: 'Sistem tarafından yönetilir', fanMode: 'Donanım tarafından yönetilir' },
  { id: 'power-saver', name: 'Güç Tasarrufu', description: 'power-profiles-daemon güç tasarrufu profili.', icon: 'Leaf', cpuGovernor: 'Sistem tarafından yönetilir', fanMode: 'Donanım tarafından yönetilir' },
];
