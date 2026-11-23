import React from 'react';
import { useTheme } from '../../context/ThemeContext';
import { motion } from 'motion/react';

interface M3GaugeProps {
  value: number; // 0 to 100
  title: string;
  subtitle?: string;
  size?: number;
  strokeWidth?: number;
  unit?: string;
  color?: string;
  icon?: React.ReactNode;
  statusText?: string;
  id?: string;
}

export const M3Gauge: React.FC<M3GaugeProps> = ({
  value,
  title,
  subtitle,
  size = 130,
  strokeWidth = 10,
  unit = '%',
  color,
  icon,
  statusText,
  id,
}) => {
  const { colors, isDark } = useTheme();

  const safeVal = Number.isFinite(value) ? value : 0;
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference - (Math.min(100, Math.max(0, safeVal)) / 100) * circumference;

  const activeColor = color || colors.primary;

  return (
    <div id={id} className="flex flex-col items-center justify-center relative select-none">
      <div className="relative flex items-center justify-center" style={{ width: size, height: size }}>
        {/* SVG Circle Gauge */}
        <svg
          className="transform -rotate-90 origin-center"
          width={size}
          height={size}
        >
          {/* Background Track */}
          <circle
            cx={size / 2}
            cy={size / 2}
            r={radius}
            stroke={colors.surfaceContainerHighest}
            strokeWidth={strokeWidth}
            fill="transparent"
          />

          {/* Active Animated Arc */}
          <motion.circle
            cx={size / 2}
            cy={size / 2}
            r={radius}
            stroke={activeColor}
            strokeWidth={strokeWidth}
            strokeDasharray={circumference}
            initial={{ strokeDashoffset: circumference }}
            animate={{ strokeDashoffset }}
            transition={{ duration: 0.8, ease: [0.2, 0, 0, 1] }}
            strokeLinecap="round"
            fill="transparent"
          />
        </svg>

        {/* Center Content */}
        <div className="absolute inset-0 flex flex-col items-center justify-center text-center p-2">
          {icon && <div className="mb-0.5 opacity-80">{icon}</div>}
          <div className="flex items-baseline justify-center">
            <span
              className="text-xl font-extrabold tracking-tight font-mono"
              style={{ color: isDark ? '#FFFFFF' : '#1A1C1E' }}
            >
              {statusText ? statusText : Math.round(safeVal)}
            </span>
            {!statusText && (
              <span className="text-xs font-semibold ml-0.5 opacity-70">
                {unit}
              </span>
            )}
          </div>
          <span
            className="text-[11px] font-semibold uppercase tracking-wider mt-0.5"
            style={{ color: colors.outline }}
          >
            {title}
          </span>
        </div>
      </div>

      {subtitle && (
        <span
          className="text-xs font-medium mt-1.5 opacity-80 text-center font-mono"
          style={{ color: isDark ? '#C7C6CA' : '#49454F' }}
        >
          {subtitle}
        </span>
      )}
    </div>
  );
};
