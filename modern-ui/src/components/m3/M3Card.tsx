import React from 'react';
import { HTMLMotionProps } from 'motion/react';
import { useTheme } from '../../context/ThemeContext';
interface Props extends HTMLMotionProps<'div'> { variant?: 'elevated' | 'filled' | 'outlined'; hoverable?: boolean; children: React.ReactNode }
export function M3Card({ children, className = '', onClick, id, variant = 'filled' }: Props) {
 const { colors } = useTheme();
 return <div id={id} className={'m3-card ' + className} style={{ backgroundColor: variant === 'outlined' ? colors.surface : colors.surfaceContainer, borderColor: colors.outlineVariant }} onClick={onClick} role={onClick ? 'button' : undefined} tabIndex={onClick ? 0 : undefined} onKeyDown={e => { if (onClick && (e.key === 'Enter' || e.key === ' ')) { e.preventDefault(); onClick(e as unknown as React.MouseEvent<HTMLDivElement>); } }}>{children}</div>;
}
