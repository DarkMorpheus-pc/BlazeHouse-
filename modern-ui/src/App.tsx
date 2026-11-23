import React, { useEffect, useState } from 'react';
import { ThemeProvider, useTheme } from './context/ThemeContext';
import { NavTabId } from './types';
import { M3WindowBar } from './components/m3/M3WindowBar';
import { M3NavRail } from './components/m3/M3NavRail';
import { M3SearchModal } from './components/m3/M3SearchModal';
import { M3TerminalModal } from './components/m3/M3TerminalModal';

// Views
import { HomeView } from './views/HomeView';
import { SystemDetailsView } from './views/SystemDetailsView';
import { ZramView } from './views/ZramView';
import { SystemUpdateView } from './views/SystemUpdateView';
import { StorageView } from './views/StorageView';
import { PerformanceGamingView } from './views/PerformanceGamingView';
import { AppearanceView } from './views/AppearanceView';
import { PowerBatteryView } from './views/PowerBatteryView';
import { ErrorBoundary } from './components/ErrorBoundary';
import { fetchLiveTelemetry, fetchSystemHardware, fetchZramStatus, preloadSystemData } from './services/tauriBridge';

import { motion, AnimatePresence } from 'motion/react';

const AppContent: React.FC = () => {
  const { colors, isDark, distro } = useTheme();
  const [activeTab, setActiveTab] = useState<NavTabId>('home');
  const [isNavExpanded, setIsNavExpanded] = useState<boolean>(false);
  const [isSearchOpen, setIsSearchOpen] = useState<boolean>(false);
  const [isFullscreen, setIsFullscreen] = useState<boolean>(false);
  const [isRefreshing, setIsRefreshing] = useState(false);

  useEffect(() => {
    void preloadSystemData(distro);
  }, [distro]);

  const handleRefreshSystemData = async () => {
    setIsRefreshing(true);
    try {
      await preloadSystemData(distro, true);
      window.dispatchEvent(new CustomEvent('blazehouse:data-refreshed'));
    } finally {
      setIsRefreshing(false);
    }
  };

  useEffect(() => {
    const handleSearchKey = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') {
        event.preventDefault(); setIsSearchOpen(value => !value);
      }
    };
    window.addEventListener('keydown', handleSearchKey);
    return () => window.removeEventListener('keydown', handleSearchKey);
  }, []);

  // Terminal Modal state
  const [terminalState, setTerminalState] = useState<{
    isOpen: boolean;
    title: string;
    command: string;
    steps: string[];
  }>({
    isOpen: false,
    title: '',
    command: '',
    steps: [],
  });

  const handleOpenTerminalAction = (title: string, command: string, steps: string[]) => {
    setTerminalState({
      isOpen: true,
      title,
      command,
      steps,
    });
  };

  const handleOpenDefaultTerminal = async () => {
    try {
      const [hardware, telemetry, zram] = await Promise.all([
        fetchSystemHardware(distro), fetchLiveTelemetry(true), fetchZramStatus(),
      ]);
      handleOpenTerminalAction('Canlı Sistem Teşhisi', 'Blaze House yerel sistem sorgusu', [
        `${hardware.distroName} ${hardware.distroVersion} • ${hardware.kernelVersion}`,
        `CPU: ${hardware.cpuName} • yük %${telemetry.cpuUsagePct.toFixed(1)} • ${telemetry.cpuTempC.toFixed(1)}°C`,
        `RAM: ${telemetry.ramUsageGiB.toFixed(1)} / ${hardware.totalRamGiB.toFixed(1)} GiB`,
        `ZRAM: ${zram.enabled ? `${zram.allocatedSizeGiB} GiB ${zram.algorithm}, ${zram.compressionRatio}x` : 'devre dışı'}`,
        `Disk doluluğu: %${telemetry.diskUsagePct.toFixed(1)}`,
      ]);
    } catch (e) {
      handleOpenTerminalAction('Sistem Teşhisi Başarısız', 'Blaze House yerel sistem sorgusu', [`HATA: ${e instanceof Error ? e.message : String(e)}`]);
    }
  };

  const toggleFullscreen = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => {});
      setIsFullscreen(true);
    } else {
      document.exitFullscreen().catch(() => {});
      setIsFullscreen(false);
    }
  };

  return (
    <div
      className="w-full h-screen flex flex-col overflow-hidden transition-colors duration-300 select-none font-sans"
      style={{
        backgroundColor: colors.surface,
        color: isDark ? '#E2E2E6' : '#1D1B20',
      }}
    >
      {/* 🐧 Linux Top Window Bar */}
      <M3WindowBar
        onOpenSearch={() => setIsSearchOpen(true)}
        onOpenTerminal={handleOpenDefaultTerminal}
        onRefresh={handleRefreshSystemData}
        isRefreshing={isRefreshing}
        isFullscreen={isFullscreen}
        onToggleFullscreen={toggleFullscreen}
      />

      {/* 📱 Main Body Layout: Left M3 Navigation Rail + Right Active View */}
      <div className="flex-1 flex overflow-hidden relative">
        {/* Material 3 Nav Rail */}
        <M3NavRail
          activeTab={activeTab}
          onSelectTab={(tab) => setActiveTab(tab)}
          isExpanded={isNavExpanded}
          onToggleExpanded={() => setIsNavExpanded(!isNavExpanded)}
        />

        {/* Dynamic Content View Area with Material 3 Fluid Transitions */}
        <main
          className="flex-1 h-full overflow-y-auto overflow-x-hidden p-4 sm:p-6 lg:p-8 relative"
          style={{
            backgroundColor: colors.surface,
          }}
        >
          <ErrorBoundary fallbackTitle="Görünüm Yüklenemedi">
            <>
              <div className="w-full pb-12">
                {activeTab === 'home' && (
                  <HomeView
                    onNavigate={(tab) => setActiveTab(tab)}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

                {activeTab === 'system-details' && (
                  <SystemDetailsView
                    onBack={() => setActiveTab('home')}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

                {activeTab === 'zram' && (
                  <ZramView
                    onBack={() => setActiveTab('home')}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

                {activeTab === 'updates' && (
                  <SystemUpdateView
                    onBack={() => setActiveTab('home')}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

                {activeTab === 'storage' && (
                  <StorageView
                    onBack={() => setActiveTab('home')}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

                {activeTab === 'gaming' && (
                  <PerformanceGamingView
                    onBack={() => setActiveTab('home')}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

                {activeTab === 'appearance' && (
                  <AppearanceView onBack={() => setActiveTab('home')} />
                )}

                {activeTab === 'power' && (
                  <PowerBatteryView
                    onBack={() => setActiveTab('home')}
                    onOpenTerminalAction={handleOpenTerminalAction}
                  />
                )}

              </div>
            </>
          </ErrorBoundary>
        </main>
      </div>

      {/* 🔍 Global Search Modal (Ctrl+K) */}
      <M3SearchModal
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
        onSelectTab={(tab) => setActiveTab(tab)}
      />

      {/* 💻 Linux Terminal Action Sheet Modal */}
      <M3TerminalModal
        isOpen={terminalState.isOpen}
        onClose={() => setTerminalState({ ...terminalState, isOpen: false })}
        title={terminalState.title}
        command={terminalState.command}
        steps={terminalState.steps}
      />
    </div>
  );
};

export default function App() {
  return (
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  );
}
