import React from 'react';
import { NavTabId } from '../../types';
import { useTheme } from '../../context/ThemeContext';
import {
  Home,
  Gauge,
  Cpu,
  RefreshCw,
  HardDrive,
  Gamepad2,
  Palette,
  BatteryCharging,
  ChevronLeft,
  ChevronRight,
  Settings,
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface NavItem {
  id: NavTabId;
  label: string;
  sublabel: string;
  icon: React.ElementType;
  badge?: string | number;
  badgeColor?: string;
}

const NAV_ITEMS: NavItem[] = [
  { id: 'home', label: 'Ana Sayfa', sublabel: 'Genel Bakış & Hızlı Ayarlar', icon: Home },
  { id: 'system-details', label: 'Sistem Detayları', sublabel: 'Canlı Donanım & Çekirdek', icon: Cpu },
  { id: 'zram', label: 'ZRAM Ayarları', sublabel: 'Sıkıştırılmış Bellek Takası', icon: Gauge },
  { id: 'updates', label: 'Sistem Güncelleme', sublabel: 'APT / DNF / Pacman', icon: RefreshCw },
  { id: 'storage', label: 'Depolama & Temizlik', sublabel: 'Disk Analizi & Önbellek', icon: HardDrive },
  { id: 'gaming', label: 'Performans', sublabel: 'İşlemci ve oyun ayarları', icon: Gamepad2 },
  { id: 'appearance', label: 'Görünüm', sublabel: 'Tema ve metin boyutu', icon: Palette },
  { id: 'power', label: 'Güç & Sıcaklık', sublabel: 'Güç profili ve şarj sınırı', icon: BatteryCharging },
];

interface M3NavRailProps {
  activeTab: NavTabId;
  onSelectTab: (tab: NavTabId) => void;
  isExpanded: boolean;
  onToggleExpanded: () => void;
}

export const M3NavRail: React.FC<M3NavRailProps> = ({
  activeTab,
  onSelectTab,
  isExpanded,
  onToggleExpanded,
}) => {
  const { colors, isDark } = useTheme();

  return (
    <motion.aside
      initial={false}
      animate={{ width: isExpanded ? 240 : 76 }}
      transition={{ type: 'spring', stiffness: 350, damping: 30 }}
      className="h-full flex flex-col justify-between py-3 border-r select-none z-20 shrink-0 relative transition-colors duration-300"
      style={{
        backgroundColor: colors.surfaceContainerLow,
        borderColor: colors.outlineVariant + '33',
      }}
    >
      {/* Top Section: Logo & Navigation Items */}
      <div className="flex flex-col gap-2 w-full px-2">
        {/* Logo / Brand Header inside Rail */}
        <div className="flex items-center justify-center py-2 mb-1 px-1">
          <div
            className="w-9 h-9 rounded-lg flex items-center justify-center border-t border-white/20 shadow-sm cursor-pointer transition-transform hover:scale-105 active:scale-95 group relative"
            style={{
              backgroundColor: colors.primaryContainer,
              color: colors.onPrimaryContainer,
            }}
            onClick={() => onSelectTab('home')}
            title="BlazeOS SolarEvolution"
          >
            <Settings className="w-6 h-6 transition-transform duration-300 group-hover:rotate-90" strokeWidth={2.2} />
            
          </div>
          {isExpanded && (
            <motion.div
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -10 }}
              className="ml-3 overflow-hidden flex flex-col"
            >
              <span className="font-extrabold text-sm tracking-tight" style={{ color: isDark ? '#FFF' : '#111' }}>
                BlazeOS
              </span>
              <span className="text-[11px] font-medium opacity-60">Control Center</span>
            </motion.div>
          )}
        </div>

        {/* Divider */}
        <div
          className="h-[1px] w-full my-1 opacity-20"
          style={{ backgroundColor: colors.outline }}
        />

        {/* Navigation Item Buttons */}
        <nav className="flex flex-col gap-1.5 w-full">
          {NAV_ITEMS.map((item) => {
            const isActive = activeTab === item.id;
            const Icon = item.icon;

            return (
              <button
                key={item.id}
                id={`nav-item-${item.id}`}
                onClick={() => onSelectTab(item.id)}
                className={`group relative flex items-center h-11 rounded-lg transition-all duration-120 outline-none ${
                  isExpanded ? 'px-3 justify-start' : 'justify-center w-12 mx-auto'
                }`}
                title={!isExpanded ? `${item.label} (${item.sublabel})` : undefined}
              >
                {/* Active Indicator Pill (M3 Pill shape) */}
                {isActive && (
                  <motion.div
                    layoutId="m3-nav-pill"
                    className="absolute inset-0 rounded-lg border-t border-white/15 z-0"
                    style={{
                      backgroundColor: colors.primaryContainer,
                    }}
                    transition={{ type: 'spring', stiffness: 450, damping: 32 }}
                  />
                )}

                {/* State Layer (Hover ripple feedback) */}
                <div
                  className="absolute inset-0 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-0"
                  style={{
                    backgroundColor: isActive
                      ? colors.onPrimaryContainer + '18'
                      : colors.outlineVariant + '24',
                  }}
                />

                {/* Icon Container with active transition */}
                <div
                  className={`relative z-10 flex items-center justify-center transition-all duration-200 ${
                    isActive ? 'scale-110' : 'group-hover:scale-105'
                  }`}
                  style={{
                    color: isActive ? colors.onPrimaryContainer : colors.outline,
                  }}
                >
                  <Icon className="w-5 h-5" />

                  {/* Badge on collapsed view */}
                  {!isExpanded && item.badge && (
                    <span
                      className={`absolute -top-1 -right-1 text-[9px] font-bold px-1.5 py-0.2 rounded-full shadow-sm ${
                        item.badgeColor || 'bg-amber-500 text-black'
                      }`}
                    >
                      {item.badge}
                    </span>
                  )}
                </div>

                {/* Expanded Text Label */}
                <AnimatePresence>
                  {isExpanded && (
                    <motion.div
                      initial={{ opacity: 0, x: -10 }}
                      animate={{ opacity: 1, x: 0 }}
                      exit={{ opacity: 0, x: -10 }}
                      className="relative z-10 ml-3 flex-1 flex items-center justify-between overflow-hidden text-left"
                    >
                      <div className="flex flex-col truncate pr-1">
                        <span
                          className={`text-xs font-semibold leading-tight truncate ${
                            isActive ? 'font-bold' : ''
                          }`}
                          style={{
                            color: isActive
                              ? colors.onPrimaryContainer
                              : isDark
                              ? '#E2E2E6'
                              : '#1F1F24',
                          }}
                        >
                          {item.label}
                        </span>
                        <span
                          className="text-[10px] truncate opacity-60"
                          style={{ color: colors.outline }}
                        >
                          {item.sublabel}
                        </span>
                      </div>

                      {item.badge && (
                        <span
                          className={`text-[10px] font-bold px-1.5 py-0.5 rounded-full shrink-0 ${
                            item.badgeColor || 'bg-amber-500 text-black'
                          }`}
                        >
                          {item.badge}
                        </span>
                      )}
                    </motion.div>
                  )}
                </AnimatePresence>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Bottom Rail Action: Collapse / Expand Toggle */}
      <div className="px-2 w-full flex flex-col gap-2">
        <button
          id="nav-rail-collapse-toggle"
          onClick={onToggleExpanded}
          className={`flex items-center h-9 rounded-lg transition-all duration-120 border group ${
            isExpanded ? 'px-3 justify-between w-full' : 'justify-center w-12 mx-auto'
          }`}
          style={{
            backgroundColor: colors.surfaceContainer,
            borderColor: colors.outlineVariant + '33',
            color: colors.outline,
          }}
          title={isExpanded ? 'Menüyü Daralt' : 'Menüyü Genişlet'}
        >
          {isExpanded ? (
            <>
              <span className="text-xs font-medium">Menüyü Daralt</span>
              <ChevronLeft className="w-4 h-4 text-inherit group-hover:-translate-x-0.5 transition-transform" />
            </>
          ) : (
            <ChevronRight className="w-4 h-4 text-inherit group-hover:translate-x-0.5 transition-transform" />
          )}
        </button>
      </div>
    </motion.aside>
  );
};
