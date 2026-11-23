import React, { useId } from 'react';
interface Props { value: number; onChange: (value: number) => void; min: number; max: number; step?: number; unit?: string; id?: string; label?: string; ticks?: number[]; disabled?: boolean }
export function M3Slider({ value, onChange, min, max, step = 1, unit = '', id, label, disabled }: Props) {
  const generated = useId();
  const inputId = id || generated;
  return <div className="slider-field"><label htmlFor={inputId}>{label || 'Değer'}<output>{value} {unit}</output></label>
    <input id={inputId} type="range" min={min} max={Math.max(min, max)} step={step} value={value} disabled={disabled} onChange={e => onChange(Number(e.target.value))} />
  </div>;
}
