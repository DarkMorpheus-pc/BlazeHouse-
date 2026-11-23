import React from 'react';
import { useTheme } from '../../context/ThemeContext';
import { motion } from 'motion/react';

export interface SegmentOption<T extends string> {
  value: T;
  label: string;
  icon?: React.ReactNode;
}

interface M3SegmentedButtonProps<T extends string> {
  options: SegmentOption<T>[];
  value: T;
  onChange: (val: T) => void;
  id?: string;
  className?: string;
}

export function M3SegmentedButton<T extends string>({
  options,
  value,
  onChange,
  id,
  className = '',
}: M3SegmentedButtonProps<T>) {
  const { colors, isDark } = useTheme();

  return (
    <div
      id={id}
      className={`inline-flex p-1 rounded-full border relative overflow-hidden select-none ${className}`}
      style={{
        backgroundColor: colors.surfaceContainer,
        borderColor: colors.outlineVariant + '55',
      }}
    >
      {options.map((option) => {
        const isSelected = value === option.value;

        return (
          <button
            key={option.value}
            id={`segment-btn-${option.value}`}
            type="button"
            onClick={() => onChange(option.value)}
            className="relative px-4 py-1.5 rounded-full text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors z-10 outline-none"
            style={{
              color: isSelected
                ? colors.onSecondaryContainer
                : colors.outline,
            }}
          >
            {isSelected && (
              <motion.div
                layoutId={`m3-segment-pill-${id || 'group'}`}
                className="absolute inset-0 rounded-full z-0 shadow-sm"
                style={{
                  backgroundColor: colors.secondaryContainer,
                }}
                transition={{ type: 'spring', stiffness: 450, damping: 30 }}
              />
            )}
            <span className="relative z-10 flex items-center gap-1.5">
              {option.icon}
              {option.label}
            </span>
          </button>
        );
      })}
    </div>
  );
}
