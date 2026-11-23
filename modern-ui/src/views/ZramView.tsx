import React, { useEffect, useState } from 'react';
import { ArrowLeft, RefreshCw, Check } from 'lucide-react';
import { fetchZramStatus, fetchSystemHardware, applyZramConfigToSystem, isTauri } from '../services/tauriBridge';
import { ZramConfig, ZramAlgorithm } from '../types';
import { M3Card } from '../components/m3/M3Card';
import { M3Slider } from '../components/m3/M3Slider';
import { M3Switch } from '../components/m3/M3Switch';
export function ZramView({ onBack, onOpenTerminalAction }: { onBack: () => void; onOpenTerminalAction: (title: string, cmd: string, logs: string[]) => void }) {
 const [status, setStatus] = useState<ZramConfig | null>(null);
 const [draft, setDraft] = useState<ZramConfig | null>(null);
 const [ram, setRam] = useState(0);
 const [busy, setBusy] = useState(false);
 const [error, setError] = useState('');
 const read = async (force = false) => {
  if (!isTauri()) { setError('ZRAM verileri masaüstü uygulamasında okunur.'); return; }
  setBusy(true);
  try {
   const [data, hw] = await Promise.all([fetchZramStatus(force), fetchSystemHardware('fedora', force)]);
   setStatus(data); setDraft(data); setRam(hw.totalRamGiB); setError('');
  } catch(e) { setError(String(e)); } finally { setBusy(false); }
 };
 useEffect(() => { 
  const onRefreshed = () => { void read(true); };
  void read(); 
  window.addEventListener('blazehouse:data-refreshed', onRefreshed);
  return () => window.removeEventListener('blazehouse:data-refreshed', onRefreshed);
 }, []);
 const apply = async () => {
  if (!draft) return;
  setBusy(true);
  try {
   const logs = await applyZramConfigToSystem(draft.enabled, draft.allocatedSizeGiB, draft.algorithm, draft.swappiness, draft.priority);
   await read(true); onOpenTerminalAction('ZRAM sonucu', 'systemd-zram-generator', logs);
  } catch(e) { setError(String(e)); onOpenTerminalAction('ZRAM uygulanamadı', 'systemd-zram-generator', ['HATA: ' + String(e)]); }
  finally { setBusy(false); }
 };
 const changed = status && draft && ['enabled', 'allocatedSizeGiB', 'algorithm', 'swappiness', 'priority'].some(key => status[key as keyof ZramConfig] !== draft[key as keyof ZramConfig]);
 return <div className="view-stack"><div className="page-heading"><div><button className="text-button" onClick={onBack}><ArrowLeft size={16} /> Genel bakış</button><h1>Bellek sıkıştırma</h1><p>ZRAM: çekirdek durumu ve takas ayarları.</p></div><button className="tonal-button" disabled={busy} onClick={() => read(true)}><RefreshCw size={16} /> Yenile</button></div>
 {error && <p role="alert" className="status-note">{error}</p>}{busy && <p role="status">İşlem sürüyor…</p>}
 {status && <M3Card><div className="setting-row"><h2>Mevcut durum</h2><span className="status-chip">{status.enabled ? 'Etkin' : 'Devre dışı'}</span></div><div className="summary-grid">
 <div><span>AYRILAN TAKAS</span><strong>{status.allocatedSizeGiB.toFixed(1)} GiB</strong></div><div><span>FİZİKSEL RAM</span><strong>{ram.toFixed(1)} GiB</strong></div><div><span>ALGORİTMA</span><strong>{status.algorithm}</strong></div>
 <div><span>SIKIŞTIRMA ORANI</span><strong>{status.actualUsedMiB > 0 ? status.compressionRatio.toFixed(2) + '×' : 'Henüz veri yok'}</strong></div><div><span>HAM VERİ</span><strong>{status.actualUsedMiB} MiB</strong></div><div><span>SIKIŞTIRILMIŞ VERİ</span><strong>{status.compressedUsedMiB} MiB</strong></div></div></M3Card>}
 {draft && <M3Card><div className="setting-row"><div><h2>Yapılandırma</h2><p className="support-text">Değişiklikler Uygula ile kaydedilir. ZRAM yeniden başlatılırken mevcut takas verisi RAM'e taşınır.</p></div><M3Switch ariaLabel="ZRAM etkinleştir" checked={draft.enabled} disabled={busy} onChange={enabled => setDraft({ ...draft, enabled, allocatedSizeGiB: draft.allocatedSizeGiB || Math.max(.5, Math.min(4, ram / 2)) })} /></div>
 <div className="view-stack">
 <M3Slider label="Ayrılan boyut" value={draft.allocatedSizeGiB} min={.5} max={Math.max(.5, ram, draft.allocatedSizeGiB)} step={.5} unit="GiB" disabled={busy || !draft.enabled} onChange={allocatedSizeGiB => setDraft({ ...draft, allocatedSizeGiB })} />
 <label>Algoritma <select value={draft.algorithm} disabled={busy || !draft.enabled} onChange={e => setDraft({ ...draft, algorithm: e.target.value as ZramAlgorithm })}>{['zstd','lz4','lzo','deflate'].map(value => <option key={value}>{value}</option>)}</select></label>
 <M3Slider label="Swappiness" value={draft.swappiness} min={0} max={200} disabled={busy || !draft.enabled} onChange={swappiness => setDraft({ ...draft, swappiness })} />
 <M3Slider label="Takas önceliği" value={draft.priority} min={0} max={32767} disabled={busy || !draft.enabled} onChange={priority => setDraft({ ...draft, priority })} />
 </div><div className="setting-actions"><button className="primary-button" disabled={busy || !changed} onClick={apply}><Check size={16} /> Uygula</button>{changed && <span className="status-chip">Kaydedilmemiş değişiklikler</span>}</div></M3Card>}
 </div>;
}
