import React, { useEffect, useState } from 'react';
import { ArrowLeft, RefreshCw, Trash2 } from 'lucide-react';
import { M3Card } from '../components/m3/M3Card';
import { RealStorageDetails } from '../types';
import { fetchStorageDetails, cleanSystemStorage, isTauri } from '../services/tauriBridge';
export function StorageView({ onBack, onOpenTerminalAction }: { onBack: () => void; onOpenTerminalAction: (title: string, cmd: string, logs: string[]) => void }) {
 const [data, setData] = useState<RealStorageDetails | null>(null);
 const [selected, setSelected] = useState<string[]>([]);
 const [busy, setBusy] = useState(false);
 const [error, setError] = useState('');
 const read = async (force = false) => {
  setBusy(true);
  try { if (!isTauri()) throw new Error('Disk verileri masaüstü uygulamasında okunur.'); setData(await fetchStorageDetails(force)); setError(''); }
  catch(e) { setError(String(e)); } finally { setBusy(false); }
 };
 useEffect(() => { 
  const onRefreshed = () => { void read(true); };
  void read(); 
  window.addEventListener('blazehouse:data-refreshed', onRefreshed);
  return () => window.removeEventListener('blazehouse:data-refreshed', onRefreshed);
 }, []);
 const clean = async () => {
  setBusy(true);
  try { const logs = await cleanSystemStorage(selected); onOpenTerminalAction('Temizlik sonucu', 'Seçilen önbellekler', logs); setSelected([]); await read(true); }
  catch(e) { onOpenTerminalAction('Temizlik tamamlanamadı', 'Seçilen önbellekler', ['HATA: ' + String(e)]); }
  finally { setBusy(false); }
 };
 return <div className="view-stack"><div className="page-heading"><div><button className="text-button" onClick={onBack}><ArrowLeft size={16} /> Genel bakış</button><h1>Depolama</h1><p>Kök dosya sistemi ve ölçülebilen önbellek alanı.</p></div><button className="tonal-button" disabled={busy} onClick={() => read(true)}><RefreshCw size={16} /> Yenile</button></div>
 {error && <p role="alert" className="status-note">{error}</p>}{busy && <p role="status">Disk işlemi sürüyor…</p>}
 {data && <><M3Card><h2>{data.diskName}</h2><div className="summary-grid"><div><span>TOPLAM</span><strong>{data.totalDiskGb} GiB</strong></div><div><span>KULLANILAN</span><strong>{data.usedDiskGb} GiB</strong></div><div><span>KULLANILABİLİR</span><strong>{data.freeDiskGb} GiB</strong></div></div></M3Card>
 <M3Card><h2>Önbellek temizliği</h2><p className="support-text">Paket önbelleği ve küçük resimler yeniden oluşturulabilir. Günlük temizliği yedi günden eski arşivleri kaldırır; gösterilen günlük boyutunun tümü silinmez.</p>
 <div className="mt-5 space-y-4">{data.categories.map(item => <label key={item.id} className="setting-row"><span className="setting-description">{item.isCleanable && <input type="checkbox" disabled={busy} checked={selected.includes(item.id)} onChange={e => setSelected(e.target.checked ? [...selected, item.id] : selected.filter(id => id !== item.id))} />}<span><strong>{item.name}</strong><span className="block support-text">{item.description}</span></span></span><span className="status-chip">{item.sizeGb >= 0 ? item.sizeGb.toFixed(1) + ' GiB' : 'Ölçülemedi'}</span></label>)}</div>
 <div className="setting-actions"><button className="primary-button" disabled={busy || selected.length === 0} onClick={clean}><Trash2 size={16} /> Seçilen önbellekleri temizle</button></div></M3Card></>}
 </div>;
}
