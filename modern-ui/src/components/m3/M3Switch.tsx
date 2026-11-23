import React from 'react';
import { useTheme } from '../../context/ThemeContext';
import { motion } from 'motion/react';
import { Check, X } from 'lucide-react';

interface M3SwitchProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  id?: string;
  showIcon?: boolean;
  ariaLabel?: string;
}

export const M3Switch: React.FC<M3SwitchProps> = ({
  checked,
  onChange,
  disabled = false,
  id,
  showIcon = true,
  ariaLabel,
}) => {
  const { colors } = useTheme();

  return (
    <button
      id={id}
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={ariaLabel}
      disabled={disabled}
      onClick={() => !disabled && onChange(!checked)}
      className={`relative inline-flex h-8 w-[52px] shrink-0 cursor-pointer rounded-full border-2 transition-colors duration-200 outline-none focus-visible:ring-2 focus-visible:ring-offset-2 ${
        disabled ? 'opacity-40 cursor-not-allowed' : ''
      }`}
      style={{
        backgroundColor: checked ? colors.primary : colors.surfaceContainerHighest,
        borderColor: checked ? colors.primary : colors.outline,
      }}
    >
      {/* Sliding Thumb */}
      <motion.span
        layout
        transition={{ type: 'spring', stiffness: 500, damping: 30 }}
        className="pointer-events-none flex h-6 w-6 items-center justify-center rounded-full shadow-sm"
        style={{
          backgroundColor: checked ? colors.onPrimary : colors.outline,
          x: checked ? 20 : 0,
        }}
      >
        {showIcon && (
          <span style={{ color: checked ? colors.primary : colors.surfaceContainerHighest }}>
            {checked ? (
              <Check className="w-3.5 h-3.5 stroke-[3]" />
            ) : (
              <X className="w-3 h-3 stroke-[3]" />
            )}
          </span>
        )}
      </motion.span>
    </button>
  );
};
