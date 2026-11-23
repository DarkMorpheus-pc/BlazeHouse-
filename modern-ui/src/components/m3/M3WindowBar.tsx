import React from 'react';
import { useTheme } from '../../context/ThemeContext';
import { 
  Minus, 
  Square, 
  X, 
  Search, 
  Moon, 
  Sun, 
  Sparkles, 
  Settings, 
  RefreshCw, 
  Terminal 
} from 'lucide-react';
import { motion } from 'motion/react';
import { closeWindow, minimizeWindow, toggleMaximizeWindow } from '../../services/tauriBridge';

interface M3WindowBarProps {
  onOpenSearch: () => void;
  onOpenTerminal: () => void;
  onRefresh: () => void;
  isRefreshing: boolean;
  isFullscreen: boolean;
  onToggleFullscreen: () => void;
}

export const M3WindowBar: React.FC<M3WindowBarProps> = ({
  onOpenSearch,
  onOpenTerminal,
  onRefresh,
  isRefreshing,
  isFullscreen,
  onToggleFullscreen,
}) => {
  const { colors, mode, setMode, isDark } = useTheme();

  return (
    <header 
      data-tauri-drag-region
      className="w-full h-12 flex items-center justify-between px-3 select-none border-b transition-colors duration-300 z-30 shrink-0 flex-nowrap overflow-hidden"
      style={{
        backgroundColor: colors.surfaceContainerLow,
        borderColor: colors.outlineVariant + '33',
      }}
    >
      {/* Left: Brand & Title */}
      <div className="flex items-center gap-2 shrink-0">
        <div 
          className="w-6 h-6 rounded-md flex items-center justify-center border-t border-white/20 shadow-sm"
          style={{ 
            backgroundColor: colors.primary,
            color: colors.onPrimary 
          }}
        >
          <Settings className="w-4 h-4" strokeWidth={2.2} />
        </div>
        <div className="flex items-center gap-2">
          <span className="font-bold text-sm tracking-tight whitespace-nowrap" style={{ color: isDark ? '#FFF' : '#111' }}>
            Blaze House
          </span>
        </div>
      </div>

      {/* Center: Search Bar trigger (Now strictly single line, no stretching or wrapping) */}
      <button
        id="quick-search-trigger"
        onClick={onOpenSearch}
        className="flex items-center gap-2 px-3 py-1 rounded-lg transition-all duration-120 border group max-w-sm w-full mx-2 shadow-sm min-w-0 flex-1 h-8 shrink"
        style={{
          backgroundColor: colors.surfaceContainer,
          borderColor: colors.outlineVariant + '44',
          color: colors.outline,
        }}
      >
        <Search className="w-3.5 h-3.5 shrink-0 text-inherit group-hover:text-current transition-colors" />
        <span className="text-xs text-left flex-1 font-medium group-hover:opacity-90 truncate whitespace-nowrap overflow-hidden">
          Ayarlarda veya komutlarda ara...
        </span>
        <kbd 
          className="text-[9px] font-mono px-1.5 py-0.5 rounded border hidden sm:inline-block shrink-0"
          style={{
            backgroundColor: colors.surfaceContainerHigh,
            borderColor: colors.outlineVariant + '66',
            color: isDark ? '#AAA' : '#555',
          }}
        >
          Ctrl+K
        </kbd>
      </button>

      {/* Right: Refresh + Mode + Terminal + Native Window Controls */}
      <div className="flex items-center gap-2 shrink-0">
        <button
          id="refresh-system-data"
          onClick={onRefresh}
          disabled={isRefreshing}
          className="p-1.5 rounded-lg transition-all duration-120 border hover:scale-105 active:scale-95 disabled:opacity-60"
          style={{
            backgroundColor: colors.surfaceContainer,
            borderColor: colors.outlineVariant + '44',
            color: colors.primary,
          }}
          title="Sistem verilerini yenile"
        >
          <RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin' : ''}`} />
        </button>

        {/* Light / Dark / OLED mode toggle */}
        <button
          id="theme-mode-toggle"
          onClick={() => {
            if (mode === 'dark') setMode('light');
            else if (mode === 'light') setMode('oled');
            else setMode('dark');
          }}
          className="p-1.5 rounded-lg transition-all duration-120 border hover:scale-105 active:scale-95"
          style={{
            backgroundColor: colors.surfaceContainer,
            borderColor: colors.outlineVariant + '44',
            color: colors.primary,
          }}
          title={`Görünüm Modu: ${mode.toUpperCase()} (Değiştirmek için tıkla)`}
        >
          {mode === 'light' ? (
            <Sun className="w-4 h-4" />
          ) : mode === 'oled' ? (
            <Sparkles className="w-4 h-4 text-purple-400" />
          ) : (
            <Moon className="w-4 h-4" />
          )}
        </button>

        {/* Quick Terminal Launcher */}
        <button
          id="quick-terminal-btn"
          onClick={onOpenTerminal}
          className="p-1.5 rounded-lg transition-all duration-120 border hover:scale-105 active:scale-95"
          style={{
            backgroundColor: colors.primaryContainer,
            borderColor: colors.primary + '44',
            color: colors.onPrimaryContainer,
          }}
          title="Sistem raporunu aç"
        >
          <Terminal className="w-4 h-4" />
        </button>

        {/* Real Linux Window Controls */}
        <div className="flex items-center gap-0.5 ml-1 border-l pl-2 border-white/10">
          <button
            onClick={minimizeWindow}
            className="p-1.5 rounded-md hover:bg-white/10 transition-colors opacity-70 hover:opacity-100"
            title="Simge Durumuna Küçült"
          >
            <Minus className="w-3.5 h-3.5" />
          </button>
          <button
            onClick={() => {
              toggleMaximizeWindow();
              onToggleFullscreen();
            }}
            className="p-1.5 rounded-md hover:bg-white/10 transition-colors opacity-70 hover:opacity-100"
            title={isFullscreen ? 'Pencereyi Küçült' : 'Tam Ekran'}
          >
            <Square className="w-3 h-3" />
          </button>
          <button
            onClick={closeWindow}
            className="p-1.5 rounded-md hover:bg-red-500/20 hover:text-red-400 transition-colors opacity-70 hover:opacity-100"
            title="Kapat"
          >
            <X className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </header>
  );
};
