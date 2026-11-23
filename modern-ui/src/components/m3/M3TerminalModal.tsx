import React, { useEffect, useState, useRef } from 'react';
import { useTheme } from '../../context/ThemeContext';
import { motion, AnimatePresence } from 'motion/react';
import { Terminal, X, CheckCircle2, ShieldAlert, Sparkles, Copy, Check } from 'lucide-react';

interface M3TerminalModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  command: string;
  steps: string[];
  onComplete?: () => void;
}

export const M3TerminalModal: React.FC<M3TerminalModalProps> = ({
  isOpen,
  onClose,
  title,
  command,
  steps,
  onComplete,
}) => {
  const { colors, isDark } = useTheme();
  const [currentStepIndex, setCurrentStepIndex] = useState(0);
  const [isFinished, setIsFinished] = useState(false);
  const [copied, setCopied] = useState(false);
  const terminalLogsRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) {
      setCurrentStepIndex(0);
      setIsFinished(false);
      return;
    }

    if (currentStepIndex < steps.length) {
      setCurrentStepIndex(steps.length);
    } else if (!isFinished) {
      setIsFinished(true);
      if (onComplete) onComplete();
    }
  }, [isOpen, currentStepIndex, steps.length, isFinished, onComplete]);

  useEffect(() => {
    if (terminalLogsRef.current) {
      terminalLogsRef.current.scrollTop = terminalLogsRef.current.scrollHeight;
    }
  }, [currentStepIndex]);

  const copyLogs = async () => {
    const text = `$ ${command}\n` + steps.slice(0, currentStepIndex).join('\n');
    try { await navigator.clipboard.writeText(text); } catch { setCopied(false); return; }
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isOpen) return null;
  const hasError = steps.some((step) => step.startsWith('HATA'));

  return (
    <AnimatePresence>
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80">
        <motion.div
          initial={{ opacity: 0, scale: 0.94, y: 15 }}
          animate={{ opacity: 1, scale: 1, y: 0 }}
          exit={{ opacity: 0, scale: 0.94, y: 15 }}
          transition={{ duration: 0.25, ease: [0.2, 0, 0, 1] }}
          className="w-full max-w-2xl rounded-3xl overflow-hidden shadow-2xl border flex flex-col"
          style={{
            backgroundColor: colors.surfaceContainerLow,
            borderColor: colors.outlineVariant + '66',
          }}
        >
          {/* Terminal Window Header */}
          <div
            className="flex items-center justify-between px-5 py-3.5 border-b select-none"
            style={{
              backgroundColor: colors.surfaceContainerHigh,
              borderColor: colors.outlineVariant + '44',
            }}
          >
            <div className="flex items-center gap-3">
              <div
                className="w-8 h-8 rounded-xl flex items-center justify-center"
                style={{
                  backgroundColor: colors.primaryContainer,
                  color: colors.onPrimaryContainer,
                }}
              >
                <Terminal className="w-4 h-4" />
              </div>
              <div className="flex flex-col">
                <span className="text-sm font-bold tracking-tight" style={{ color: isDark ? '#FFF' : '#111' }}>
                  {title}
                </span>
                <span className="text-[11px] font-mono opacity-60">
                  Sistemden okunan bilgiler ve işlem sonuçları
                </span>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <button
                onClick={copyLogs}
                className="p-1.5 rounded-full hover:bg-white/10 transition-colors opacity-70 hover:opacity-100"
                title="Çıktıyı Kopyala"
              >
                {copied ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
              </button>
              <button
                onClick={onClose}
                className="p-1.5 rounded-full hover:bg-white/10 transition-colors"
                title="Kapat"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
          </div>

          {/* Terminal Body */}
          <div
            ref={terminalLogsRef}
            className="p-5 font-mono text-xs h-80 overflow-y-auto space-y-2 bg-[#0B0C0E] text-zinc-300"
          >
            {/* Command prompt banner */}
            <div className="flex items-center gap-2 text-emerald-400 font-bold pb-2 border-b border-zinc-800">
              <span className="text-amber-400">blaze-house$</span>
              <span className="text-white">{command}</span>
            </div>

            {/* Output Steps */}
            {steps.slice(0, currentStepIndex).map((step, idx) => (
              <motion.div
                key={idx}
                initial={{ opacity: 0, x: -6 }}
                animate={{ opacity: 1, x: 0 }}
                className="flex items-start gap-2"
              >
                <span className="text-zinc-600 select-none">[{idx + 1}/{steps.length}]</span>
                <span className={step.includes('BAŞARILI') || step.includes('Tamamlandı') ? 'text-emerald-300 font-semibold' : 'text-zinc-300'}>
                  {step}
                </span>
              </motion.div>
            ))}

            {/* Live blinking cursor if active */}
            {!isFinished && (
              <div className="flex items-center gap-2 text-amber-400 animate-pulse pt-2">
                <span className="inline-block w-2 h-4 bg-amber-400" />
                <span className="text-[11px]">İşlem yürütülüyor, lütfen bekleyin...</span>
              </div>
            )}

            {/* Finished badge */}
            {isFinished && (
              <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                className="mt-4 p-3 rounded-2xl bg-emerald-950/40 border border-emerald-500/30 flex items-center justify-between"
              >
                <div className="flex items-center gap-2 text-emerald-400">
                  {hasError ? <ShieldAlert className="w-4 h-4" /> : <CheckCircle2 className="w-4 h-4" />}
                  <span className="font-bold text-xs">{hasError ? 'İşlem hata ile sonuçlandı' : 'İşlem tamamlandı'}</span>
                </div>
                <span className="text-[11px] font-mono text-emerald-300/70">RAPOR</span>
              </motion.div>
            )}
          </div>

          {/* Terminal Footer */}
          <div
            className="flex items-center justify-between px-5 py-3 border-t"
            style={{
              backgroundColor: colors.surfaceContainerHigh,
              borderColor: colors.outlineVariant + '44',
            }}
          >
            <div className="flex items-center gap-2 text-xs opacity-70">
              <Sparkles className="w-3.5 h-3.5" style={{ color: colors.primary }} />
              <span>Blaze House · İşlem raporu</span>
            </div>

            <button
              onClick={onClose}
              className="px-5 py-1.5 rounded-full text-xs font-bold transition-all hover:scale-105 active:scale-95 shadow-md"
              style={{
                backgroundColor: isFinished ? colors.primary : colors.surfaceContainerHighest,
                color: isFinished ? colors.onPrimary : colors.outline,
              }}
            >
              {isFinished ? 'Kapat' : 'Arka Plana Al'}
            </button>
          </div>
        </motion.div>
      </div>
    </AnimatePresence>
  );
};
