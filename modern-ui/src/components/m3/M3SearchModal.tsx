import React, { useState, useEffect } from 'react';
import { useTheme } from '../../context/ThemeContext';
import { NavTabId } from '../../types';
import { motion, AnimatePresence } from 'motion/react';
import { 
  Search, 
  X, 
  Cpu, 
  Gauge, 
  RefreshCw, 
  HardDrive, 
  Gamepad2, 
  Palette, 
  BatteryCharging, 
  ArrowRight,
  Sparkles,
  Layers
} from 'lucide-react';

interface SearchItem {
  id: string;
  title: string;
  category: string;
  tabId: NavTabId;
  icon: React.ElementType;
  keywords: string[];
}

const SEARCH_DATABASE: SearchItem[] = [
  { id: 'zram-size', title: 'ZRAM Boyutu & Sıkıştırma Algoritması (zstd/lz4)', category: 'Bellek Optimizasyonu', tabId: 'zram', icon: Gauge, keywords: ['zram', 'swap', 'ram', 'sıkıştırma', 'zstd', 'lz4', 'swappiness', 'bellek'] },
  { id: 'system-details', title: 'İşlemci, RAM ve Çekirdek (Kernel) Bilgisi', category: 'Sistem', tabId: 'system-details', icon: Cpu, keywords: ['cpu', 'işlemci', 'ram', 'donanım', 'hardware', 'çekirdek', 'kernel', 'i9', 'dell'] },
  { id: 'system-updates', title: 'Paket Güncellemeleri & Kernel Yükseltme', category: 'Yazılım', tabId: 'updates', icon: RefreshCw, keywords: ['güncelleme', 'update', 'apt', 'dnf', 'flatpak', 'upgrade', 'güvenlik', 'paket'] },
  { id: 'storage-clean', title: 'Disk Alanı Temizleyici & Önbellek Silme', category: 'Depolama', tabId: 'storage', icon: HardDrive, keywords: ['disk', 'storage', 'depolama', 'önbellek', 'cache', 'temizle', 'journal', 'log'] },
  { id: 'gaming-mode', title: 'İşlemci, Wine ve GameMode Ayarları', category: 'Performans', tabId: 'gaming', icon: Gamepad2, keywords: ['oyun', 'gaming', 'fsync', 'feral', 'gamemode', 'vrr', 'freesync', 'gpu', 'fps'] },
  { id: 'theme-colors', title: 'Uygulama Renkleri ve Tema', category: 'Kişiselleştirme', tabId: 'appearance', icon: Palette, keywords: ['tema', 'renk', 'material you', 'm3', 'dark', 'light', 'oled', 'duvar kağıdı', 'wallpaper'] },
  { id: 'power-profiles', title: 'Güç Profili ve Desteklenen Şarj Sınırları', category: 'Güç & Batarya', tabId: 'power', icon: BatteryCharging, keywords: ['güç', 'pil', 'batarya', 'battery', 'tlp', 'fan', 'sıcaklık', 'tasarruf', 'performance'] },
];

interface M3SearchModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectTab: (tab: NavTabId) => void;
}

export const M3SearchModal: React.FC<M3SearchModalProps> = ({
  isOpen,
  onClose,
  onSelectTab,
}) => {
  const { colors, isDark } = useTheme();
  const [query, setQuery] = useState('');

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        if (isOpen) onClose();
        else setQuery('');
      } else if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const filtered = SEARCH_DATABASE.filter((item) => {
    if (!query.trim()) return true;
    const q = query.toLowerCase();
    return (
      item.title.toLowerCase().includes(q) ||
      item.category.toLowerCase().includes(q) ||
      item.keywords.some((k) => k.toLowerCase().includes(q))
    );
  });

  return (
    <AnimatePresence>
      <div className="fixed inset-0 z-50 flex items-start justify-center pt-20 px-4 bg-black/75">
        <motion.div
          initial={{ opacity: 0, scale: 0.95, y: -20 }}
          animate={{ opacity: 1, scale: 1, y: 0 }}
          exit={{ opacity: 0, scale: 0.95, y: -20 }}
          transition={{ duration: 0.2, ease: [0.2, 0, 0, 1] }}
          className="w-full max-w-xl rounded-3xl overflow-hidden shadow-2xl border flex flex-col"
          style={{
            backgroundColor: colors.surfaceContainerLow,
            borderColor: colors.outlineVariant + '66',
          }}
        >
          {/* Search Input Bar */}
          <div
            className="flex items-center gap-3 px-5 py-4 border-b"
            style={{
              backgroundColor: colors.surfaceContainer,
              borderColor: colors.outlineVariant + '44',
            }}
          >
            <Search className="w-5 h-5 text-amber-400" style={{ color: colors.primary }} />
            <input
              autoFocus
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Ayarlarda, modüllerde veya donanımda ara..."
              className="flex-1 bg-transparent border-none outline-none text-sm font-medium placeholder:opacity-50"
              style={{ color: isDark ? '#FFF' : '#111' }}
            />
            {query && (
              <button
                onClick={() => setQuery('')}
                className="p-1 rounded-full hover:bg-white/10 opacity-70 hover:opacity-100"
              >
                <X className="w-4 h-4" />
              </button>
            )}
            <kbd className="text-[10px] font-mono px-2 py-0.5 rounded border opacity-60">ESC</kbd>
          </div>

          {/* Results List */}
          <div className="max-h-80 overflow-y-auto p-3 space-y-1.5">
            {filtered.length === 0 ? (
              <div className="py-8 text-center opacity-60 text-xs">
                "{query}" ile eşleşen bir ayar veya modül bulunamadı.
              </div>
            ) : (
              filtered.map((item) => {
                const Icon = item.icon;
                return (
                  <button
                    key={item.id}
                    onClick={() => {
                      onSelectTab(item.tabId);
                      onClose();
                    }}
                    className="w-full flex items-center justify-between p-3 rounded-2xl transition-all duration-150 text-left group border border-transparent hover:border-white/10"
                    style={{
                      backgroundColor: colors.surfaceContainer,
                    }}
                  >
                    <div className="flex items-center gap-3">
                      <div
                        className="w-9 h-9 rounded-xl flex items-center justify-center transition-colors group-hover:scale-105"
                        style={{
                          backgroundColor: colors.primaryContainer,
                          color: colors.onPrimaryContainer,
                        }}
                      >
                        <Icon className="w-4 h-4" />
                      </div>
                      <div>
                        <div
                          className="text-xs font-bold transition-colors group-hover:text-amber-400"
                          style={{ color: isDark ? '#FFF' : '#111' }}
                        >
                          {item.title}
                        </div>
                        <div className="text-[10px] font-medium opacity-60">
                          {item.category}
                        </div>
                      </div>
                    </div>
                    <ArrowRight
                      className="w-4 h-4 opacity-40 group-hover:opacity-100 group-hover:translate-x-1 transition-all"
                      style={{ color: colors.primary }}
                    />
                  </button>
                );
              })
            )}
          </div>

          {/* Footer */}
          <div
            className="px-5 py-2.5 border-t text-[11px] flex items-center justify-between opacity-70"
            style={{
              backgroundColor: colors.surfaceContainerHigh,
              borderColor: colors.outlineVariant + '33',
            }}
          >
            <span>BlazeOS Akıllı Sistem İndeksi</span>
            <span>Seçmek için tıkla</span>
          </div>
        </motion.div>
      </div>
    </AnimatePresence>
  );
};
