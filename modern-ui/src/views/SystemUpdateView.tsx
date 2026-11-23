import React, { useEffect, useRef, useState } from 'react';
import { ArrowLeft, RefreshCw, Download, RotateCcw, ShieldCheck, X, Terminal, Loader2, GitBranch, ArrowUpCircle } from 'lucide-react';
import { useTheme } from '../context/ThemeContext';
import { PackageUpdate, UpdateChannelInfo } from '../types';
import {
  cancelOfflineUpdate,
  executePrivilegedCommand,
  fetchSystemUpdates,
  getOfflineUpdateStatus,
  getUpdateChannel,
  OfflineUpdateProgress,
  prepareOfflineUpdate,
  prepareSystemUpgrade,
  rebootToOfflineUpdate,
  rebootToSystemUpgrade,
  subscribeOfflineUpdateProgress,
} from '../services/tauriBridge';
import { M3Card } from '../components/m3/M3Card';

type Props = {
  onBack: () => void;
  onOpenTerminalAction: (title: string, cmd: string, logs: string[]) => void;
};

export function SystemUpdateView({ onBack, onOpenTerminalAction }: Props) {
  const { distro, colors } = useTheme();
  const [updates, setUpdates] = useState<PackageUpdate[] | null>(null);
  const [channelInfo, setChannelInfo] = useState<UpdateChannelInfo | null>(null);
  const [upgradeStaged, setUpgradeStaged] = useState(false);
  const [busy, setBusy] = useState(false);
  const [preparing, setPreparing] = useState(false);
  const [prepared, setPrepared] = useState(false);
  const [preparedSummary, setPreparedSummary] = useState('');
  const [progress, setProgress] = useState<OfflineUpdateProgress | null>(null);
  const [liveLines, setLiveLines] = useState<string[]>([]);
  const [lastLogs, setLastLogs] = useState<string[]>([]);
  const [error, setError] = useState('');
  const logBoxRef = useRef<HTMLDivElement>(null);

  const syncOfflineStatus = async () => {
    if (distro !== 'fedora') return;
    try {
      const st = await getOfflineUpdateStatus();
      setPrepared(st.prepared);
      setPreparedSummary(st.stateSummary || '');
      setUpgradeStaged((st.stateSummary || '').includes('ana sürüm yükseltmesi'));
    } catch {
      // ignore read-only status probe error
    }
  };

  const check = async (force = false) => {
    setBusy(true);
    setError('');
    try {
      const [list, ch] = await Promise.all([
        fetchSystemUpdates(distro, force),
        getUpdateChannel(distro),
        syncOfflineStatus(),
      ]);
      setUpdates(list);
      setChannelInfo(ch);
    } catch (e) {
      setError(String(e));
      setUpdates(null);
    } finally {
      setBusy(false);
    }
  };

  useEffect(() => {
    const onRefreshed = () => { void check(true); };
    void check();
    window.addEventListener('blazehouse:data-refreshed', onRefreshed);
    return () => window.removeEventListener('blazehouse:data-refreshed', onRefreshed);
  }, [distro]);

  useEffect(() => {
    if (logBoxRef.current) {
      logBoxRef.current.scrollTop = logBoxRef.current.scrollHeight;
    }
  }, [liveLines.length]);

  const runFedoraOfflineTask = async (
    initialLabel: string,
    summaryWhenDone: string,
    runner: () => Promise<string[]>,
    isMajor = false
  ) => {
    setBusy(true);
    setError('');
    setPreparing(true);
    setLiveLines([]);
    setProgress({
      phase: 'starting',
      label: initialLabel,
      percent: 2,
      current: 0,
      total: 0,
      package: '',
      line: '',
      done: false,
    });
    let unlisten: (() => void) | null = null;
    try {
      unlisten = await subscribeOfflineUpdateProgress((ev) => {
        setProgress(ev);
        if (ev.line) {
          setLiveLines((prev) => {
            if (prev[prev.length - 1] === ev.line) return prev;
            const next = [...prev, ev.line];
            return next.length > 120 ? next.slice(next.length - 120) : next;
          });
        }
      });
      const logs = await runner();
      setLastLogs(logs);
      setPrepared(true);
      setUpgradeStaged(isMajor);
      setPreparedSummary(summaryWhenDone);
    } catch (e) {
      setError(String(e));
    } finally {
      if (unlisten) unlisten();
      setPreparing(false);
      setBusy(false);
      void syncOfflineStatus();
    }
  };

  const apply = async () => {
    if (distro === 'fedora') {
      await runFedoraOfflineTask(
        'Yetkilendirme ve DNF5 hazırlanıyor...',
        'Çevrimdışı güncelleme indirildi ve yeniden başlatmada kuruluma hazır.',
        prepareOfflineUpdate,
        false
      );
      return;
    }

    setBusy(true);
    setError('');
    const command = distro === 'arch' ? 'pacman -Syu --noconfirm' : 'apt-get update && apt-get upgrade -y';
    try {
      const logs = await executePrivilegedCommand(command);
      onOpenTerminalAction('Paket yöneticisi sonucu', command, logs);
      await check(true);
    } catch (e) {
      onOpenTerminalAction('Güncelleme başarısız', command, ['HATA: ' + String(e)]);
    } finally {
      setBusy(false);
    }
  };

  const startMajorUpgrade = async (targetVer: string, seriesName: string) => {
    await runFedoraOfflineTask(
      `Fedora ${targetVer} (${seriesName}) yükseltmesi hazırlanıyor...`,
      `Fedora ${targetVer} ana sürüm yükseltmesi indirildi ve yeniden başlatmada uygulanacak.`,
      () => prepareSystemUpgrade(targetVer),
      true
    );
  };

  const reboot = async () => {
    const promptMsg = upgradeStaged
      ? 'Sistem Btrfs anlık görüntüsü korunarak yeni BlazeOS ana sürüm yükseltmesine yeniden başlatılacak. Devam edilsin mi?'
      : 'Bilgisayar çevrimdışı güncelleme ekranına yeniden başlatılacak. Devam edilsin mi?';
    if (!window.confirm(promptMsg)) return;
    setBusy(true);
    setError('');
    try {
      if (upgradeStaged) {
        await rebootToSystemUpgrade();
      } else {
        await rebootToOfflineUpdate();
      }
    } catch (e) {
      setError(String(e));
      setBusy(false);
    }
  };

  const cancelPrepared = async () => {
    setBusy(true);
    setError('');
    try {
      await cancelOfflineUpdate();
      setPrepared(false);
      setUpgradeStaged(false);
      setPreparedSummary('');
      setProgress(null);
      setLiveLines([]);
      await check(true);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const upgradeTarget = channelInfo?.upgradeTarget;
  const upgradeAvailable = Boolean(
    distro === 'fedora' &&
      upgradeTarget &&
      upgradeTarget.ready &&
      upgradeTarget.targetReleasever &&
      upgradeTarget.targetReleasever !== channelInfo?.currentReleasever
  );

  return <div className="view-stack">
    <div className="page-heading">
      <div>
        <button className="text-button" onClick={onBack}><ArrowLeft size={16} /> Genel bakış</button>
        <h1>Güncellemeler</h1>
        <p>{distro === 'fedora' ? 'Fedora ve BlazeOS güncellemeleri güvenli çevrimdışı kurulumla uygulanır.' : `${distro === 'arch' ? 'Pacman' : 'APT'} tarafından bildirilen paket sürümleri.`}</p>
      </div>
      <button className="tonal-button" disabled={busy} onClick={() => check(true)}><RefreshCw size={16} /> Yenile</button>
    </div>

    {channelInfo && <M3Card>
      <div className="setting-row">
        <div>
          <h2><GitBranch size={18} /> {channelInfo.productName} {channelInfo.productSeries} Kanalı</h2>
          <p className="support-text">
            GitHub: {channelInfo.githubRepo} • Taban Sürüm: {channelInfo.currentReleasever}
            {channelInfo.remoteSeries ? ` • Uzak Kanal: ${channelInfo.remoteSeries}` : ''}
          </p>
          <p className="support-text" style={{ marginTop: 4 }}>{channelInfo.releaseNotes}</p>
        </div>
        <span
          className="status-chip"
          style={{
            backgroundColor: channelInfo.onlineReachable ? colors.primaryContainer : colors.surfaceContainerHighest,
            color: channelInfo.onlineReachable ? colors.onPrimaryContainer : undefined,
          }}
        >
          {channelInfo.onlineReachable ? 'GitHub Kanalı Bağlı' : 'Yerel Kanal'}
        </span>
      </div>
    </M3Card>}

    {upgradeAvailable && upgradeTarget && !prepared && <M3Card>
      <div className="setting-row">
        <div>
          <h2><ArrowUpCircle size={20} /> Yeni Ana Sürüm Hazır: {channelInfo?.productName} {upgradeTarget.series}</h2>
          <p className="support-text">
            Fedora {channelInfo?.currentReleasever} → {upgradeTarget.targetReleasever} yükseltmesi öncesi otomatik Btrfs sistem anlık görüntüsü alınır.
            {upgradeTarget.notes ? ` ${upgradeTarget.notes}` : ''}
          </p>
        </div>
        <button
          className="primary-button"
          disabled={busy}
          onClick={() => startMajorUpgrade(upgradeTarget.targetReleasever, upgradeTarget.series)}
        >
          <ArrowUpCircle size={16} /> Ana Sürüme Yükselt ({upgradeTarget.series})
        </button>
      </div>
    </M3Card>}

    {error && <p role="alert" className="status-note">{error}</p>}
    {busy && !preparing && <p role="status">Depolar ve güncelleme durumu denetleniyor…</p>}

    {preparing && progress && <M3Card>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
        <div className="setting-row" style={{ alignItems: 'center' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
            <Loader2 size={18} className="animate-spin" style={{ color: colors.primary }} />
            <div>
              <h2>{progress.label}</h2>
              <p className="support-text" style={{ marginTop: 2 }}>
                {progress.total > 0 ? `[${progress.current}/${progress.total}] ` : ''}{progress.package || 'DNF5 çıktısı bekleniyor...'}
              </p>
            </div>
          </div>
          <span
            className="status-chip"
            style={{
              fontVariantNumeric: 'tabular-nums',
              fontWeight: 700,
              backgroundColor: colors.primaryContainer,
              color: colors.onPrimaryContainer,
            }}
          >
            %{progress.percent}
          </span>
        </div>

        <div
          role="progressbar"
          aria-valuenow={progress.percent}
          aria-valuemin={0}
          aria-valuemax={100}
          style={{
            width: '100%',
            height: 10,
            borderRadius: 999,
            overflow: 'hidden',
            backgroundColor: colors.surfaceContainerHighest,
          }}
        >
          <div
            style={{
              width: `${Math.max(3, Math.min(100, progress.percent))}%`,
              height: '100%',
              borderRadius: 999,
              backgroundColor: colors.primary,
              transition: 'width 220ms cubic-bezier(0.2, 0, 0, 1)',
            }}
          />
        </div>

        {liveLines.length > 0 && (
          <div
            ref={logBoxRef}
            style={{
              maxHeight: 132,
              overflowY: 'auto',
              padding: '10px 12px',
              borderRadius: 12,
              backgroundColor: 'rgba(11, 12, 14, 0.72)',
              fontFamily: 'var(--font-mono)',
              fontSize: '0.73rem',
              lineHeight: 1.45,
              color: '#d4d4d8',
              border: '1px solid rgba(255,255,255,0.08)',
            }}
          >
            {liveLines.map((ln, idx) => (
              <div key={idx} style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}>{ln}</div>
            ))}
          </div>
        )}
      </div>
    </M3Card>}

    {prepared && distro === 'fedora' && <M3Card>
      <div className="setting-row">
        <div>
          <h2><ShieldCheck size={20} /> Kuruluma hazır</h2>
          <p className="support-text">
            {preparedSummary || 'İndirilen paketler yeniden başlatma sırasında kurulacak. Ekranda gerçek işlem yüzdesi ve “Secured by Blaze” doğrulaması gösterilecek.'}
          </p>
        </div>
        <div style={{ display: 'flex', gap: 10, flexWrap: 'wrap', justifyContent: 'flex-end' }}>
          {lastLogs.length > 0 && (
            <button
              className="tonal-button"
              onClick={() => onOpenTerminalAction('Çevrimdışı güncelleme günlüğü', '/usr/libexec/blazeos-update prepare', lastLogs)}
            >
              <Terminal size={16} /> Detaylı günlük
            </button>
          )}
          <button className="tonal-button" disabled={busy} onClick={cancelPrepared}><X size={16} /> İptal et</button>
          <button className="primary-button" disabled={busy} onClick={reboot}><RotateCcw size={16} /> Şimdi yeniden başlat</button>
        </div>
      </div>
    </M3Card>}

    {updates !== null && <M3Card>
      <div className="setting-row">
        <div>
          <h2>{updates.length ? `${updates.length} paket güncellenebilir` : 'Sorguda güncelleme bulunmadı'}</h2>
          <p className="support-text">{distro === 'fedora' ? 'Paketler açık oturumda değiştirilmez; önce indirilir, sonra korumalı yeniden başlatma ortamında kurulur.' : 'Sonuç paket veritabanının durumuna bağlıdır. AUR ve Flatpak bu sorguya dahil değildir.'}</p>
        </div>
        {updates.length > 0 && !prepared && <button className="primary-button" disabled={busy} onClick={apply}><Download size={16} /> {distro === 'fedora' ? 'İndir ve hazırla' : 'Güncellemeleri uygula'}</button>}
      </div>
    </M3Card>}

    {updates?.map(pkg => <M3Card key={pkg.id}>
      <div className="setting-row"><h2>{pkg.name}</h2><span className="status-chip">{pkg.source}</span></div>
      <p className="support-text">{pkg.currentVersion} → {pkg.newVersion}</p>
    </M3Card>)}
  </div>;
}
