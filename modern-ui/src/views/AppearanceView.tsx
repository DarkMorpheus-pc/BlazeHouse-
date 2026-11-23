import React, { useEffect, useState } from 'react';
import { ArrowLeft, Check } from 'lucide-react';
import { useTheme, THEME_PALETTES } from '../context/ThemeContext';
import { M3ThemeColor, ThemeMode } from '../types';
import { M3Card } from '../components/m3/M3Card';
import { M3Slider } from '../components/m3/M3Slider';
export function AppearanceView({ onBack }: { onBack: () => void }) {
  const { colors, colorScheme, setColorScheme, mode, setMode, isDark } = useTheme();
  const [scale, setScale] = useState(() => Number(localStorage.getItem('blazehouse-font-scale')) || 100);
  useEffect(() => { document.documentElement.style.fontSize = scale + '%'; localStorage.setItem('blazehouse-font-scale', String(scale)); }, [scale]);
  return <div className="view-stack"><div className="page-heading"><div><button className="text-button" onClick={onBack}><ArrowLeft size={16} /> Genel bakış</button><h1>Görünüm</h1><p>Blaze House için sade yüzeyler ve okunabilirlik. Tercihler yalnızca bu uygulamayı etkiler.</p></div></div>
    <M3Card><h2>Vurgu rengi</h2><div className="grid grid-cols-2 sm:grid-cols-3 gap-3 mt-5">{(Object.keys(THEME_PALETTES) as M3ThemeColor[]).map(key => {
      const palette = THEME_PALETTES[key][isDark ? 'dark' : 'light'];
      return <button key={key} className="palette-button" aria-pressed={colorScheme === key} onClick={() => setColorScheme(key)} style={{ borderColor: colorScheme === key ? colors.primary : colors.outlineVariant }}><span className="w-6 h-6 rounded-full" style={{ background: palette.primary }} />{{ kineticGlass: 'Kinetic Glass', amber: 'Kehribar', blue: 'Mavi', crimson: 'Kırmızı', emerald: 'Yeşil', purple: 'Mor', neutral: 'Grafit' }[key]}{colorScheme === key && <Check size={16} />}</button>;
    })}</div></M3Card>
    <M3Card><h2>Tema</h2><div className="setting-actions">{(['light', 'dark', 'oled'] as ThemeMode[]).map(value => <button className={mode === value ? 'primary-button' : 'tonal-button'} key={value} aria-pressed={mode === value} onClick={() => setMode(value)}>{{ light: 'Açık', dark: 'Koyu', oled: 'Saf siyah' }[value]}</button>)}</div></M3Card>
    <M3Card><h2>Metin boyutu</h2><div className="mt-5"><M3Slider label="Uygulama ölçeği" value={scale} onChange={setScale} min={85} max={130} step={5} unit="%" /></div></M3Card>
  </div>;
}
