import React, { useEffect, useState } from 'react';
import { ArrowLeft, Cpu, BatteryCharging, RefreshCw, Check } from 'lucide-react';
import { M3Card } from '../components/m3/M3Card';
import { useTheme } from '../context/ThemeContext';
import { isTauri } from '../services/tauriBridge';
interface Control { id: string; label: string; description: string; group: string; value: string | null; options: string[]; supported: boolean }
interface Props { group: 'gaming' | 'power'; onBack: () => void; onOpenTerminalAction: (title: string, cmd: string, steps: string[]) => void }
export function SystemControlsView({ group, onBack, onOpenTerminalAction }: Props) {
  const { colors } = useTheme();
  const [controls, setControls] = useState<Control[]>([]);
  const [draft, setDraft] = useState<Record<string, string>>({});
  const [busy, setBusy] = useState<string | null>(null);
  const [error, setError] = useState('');
  const load = async () => {
    if (!isTauri()) { setError('Sistem ayarları masaüstü uygulamasında okunur.'); return; }
    const { invoke } = await import('@tauri-apps/api/core');
    const data = await invoke<Control[]>('get_system_controls');
    setControls(data);
    setDraft(Object.fromEntries(data.map(item => [item.id, item.value ?? ''])));
    setError('');
  };
  const refresh = async () => {
    setBusy('read');
    try { await load(); } catch (e) { setError(String(e)); } finally { setBusy(null); }
  };
  useEffect(() => { 
    const onRefreshed = () => { void refresh(); };
    void refresh(); 
    window.addEventListener('blazehouse:data-refreshed', onRefreshed);
    return () => window.removeEventListener('blazehouse:data-refreshed', onRefreshed);
  }, []);
  const apply = async (item: Control) => {
    setBusy(item.id);
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const logs = await invoke<string[]>('set_system_control', { id: item.id, value: draft[item.id] });
      await load();
      onOpenTerminalAction(item.label, 'Sistem ayarı ve doğrulama', logs);
    } catch (e) {
      onOpenTerminalAction('Ayar uygulanamadı', item.label, [`HATA: ${String(e)}`]);
      await load().catch(() => {});
    } finally { setBusy(null); }
  };
  const Icon = group === 'gaming' ? Cpu : BatteryCharging;
  return <div className="view-stack">
    <div className="page-heading"><div><button className="text-button" onClick={onBack}><ArrowLeft size={16} /> Genel bakış</button><h1>{group === 'gaming' ? 'Performans' : 'Güç ve batarya'}</h1><p>Donanımın desteklediği ayarlar ve sistemden okunan mevcut değerler.</p></div><button className="tonal-button" disabled={busy !== null} onClick={refresh}><RefreshCw size={16} /> Yenile</button></div>
    {error && <p role="alert" className="status-note">{error}</p>}
    {busy === 'read' && <p role="status">Sistem okunuyor…</p>}
    {controls.filter(item => item.group === group).map(item => <M3Card key={item.id}>
      <div className="setting-row"><div className="setting-description"><Icon size={24} style={{ color: colors.primary }} /><div><h2>{item.label}</h2><p>{item.description}</p></div></div><span className="status-chip">{item.value === null ? 'Desteklenmiyor / okunamadı' : `Mevcut: ${item.value}${item.id === 'threshold' ? '%' : ''}`}</span></div>
      {item.supported && <div className="setting-actions"><select aria-label={item.label} value={draft[item.id]} disabled={busy !== null} onChange={e => setDraft({ ...draft, [item.id]: e.target.value })}>{!item.options.includes(draft[item.id]) && <option value={draft[item.id]}>{draft[item.id]}</option>}{item.options.map(value => <option key={value} value={value}>{value}{item.id === 'threshold' ? '%' : ''}</option>)}</select><button className="primary-button" disabled={busy !== null || draft[item.id] === item.value} onClick={() => apply(item)}><Check size={16} /> {busy === item.id ? 'Uygulanıyor…' : 'Uygula'}</button></div>}
    </M3Card>)}
    {group === 'gaming' && <M3Card><h2>Oyun ve grafik desteği</h2><p className="support-text">GPU güç sınırı ve VRR, sürücüye ve masaüstü ortamına bağlıdır. Doğrulanmış denetim yolu olmayan ayarlar etkin kontrol olarak sunulmaz.</p></M3Card>}
  </div>;
}
