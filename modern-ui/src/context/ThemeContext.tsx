import React, { createContext, useContext, useEffect, useState } from 'react';
import { DistroBase, M3ThemeColor, ThemeColors, ThemeMode } from '../types';
import { fetchSystemHardware } from '../services/tauriBridge';

export const THEME_PALETTES: Record<M3ThemeColor, { name: string; distroHint?: string; dark: ThemeColors; light: ThemeColors }> = {
  kineticGlass: {
    name: 'Kinetic Glass',
    distroHint: 'BCDL Standard',
    dark: {
      id: 'kineticGlass',
      name: 'Kinetic Glass',
      primary: '#A9C7FF',
      primaryLight: '#D2E3FC',
      primaryDark: '#7CABF8',
      onPrimary: '#0D1524',
      primaryContainer: '#20252B',
      onPrimaryContainer: '#E5E9F0',
      secondary: '#9BA3AF',
      secondaryContainer: '#161A20',
      onSecondaryContainer: '#E5E9F0',
      tertiary: '#85D79E',
      tertiaryContainer: '#1B2C22',
      surface: '#0D0F12',
      surfaceDim: '#08090B',
      surfaceBright: '#262D37',
      surfaceContainerLowest: '#090B0E',
      surfaceContainerLow: '#121519',
      surfaceContainer: '#161A20',
      surfaceContainerHigh: '#20252B',
      surfaceContainerHighest: '#2A313A',
      outline: '#3B4252',
      outlineVariant: '#242933',
      glow: 'rgba(169, 199, 255, 0.15)',
    },
    light: {
      id: 'kineticGlass',
      name: 'Kinetic Glass',
      primary: '#2B5BA4',
      primaryLight: '#4B77C0',
      primaryDark: '#123D7E',
      onPrimary: '#FFFFFF',
      primaryContainer: '#E5ECF6',
      onPrimaryContainer: '#0D1B2A',
      secondary: '#5C6370',
      secondaryContainer: '#E2E6ED',
      onSecondaryContainer: '#1A1E24',
      tertiary: '#2E7D4F',
      tertiaryContainer: '#D1EEDB',
      surface: '#F5F7FA',
      surfaceDim: '#E4E7EB',
      surfaceBright: '#FFFFFF',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#ECEFF4',
      surfaceContainer: '#E5E9F0',
      surfaceContainerHigh: '#D8DEE9',
      surfaceContainerHighest: '#C9D1DF',
      outline: '#8A919E',
      outlineVariant: '#C2C8D2',
      glow: 'rgba(43, 91, 164, 0.12)',
    },
  },
  amber: {
    name: 'Blaze Flame',
    distroHint: 'BlazeOS Default',
    dark: {
      id: 'amber',
      name: 'Blaze Flame',
      primary: '#FFB77C',
      primaryLight: '#FFDCC1',
      primaryDark: '#FF7A00',
      onPrimary: '#4D2700',
      primaryContainer: '#6E3A00',
      onPrimaryContainer: '#FFDCC1',
      secondary: '#E5C0A5',
      secondaryContainer: '#59412D',
      onSecondaryContainer: '#FFDCC1',
      tertiary: '#C8CC90',
      tertiaryContainer: '#454918',
      surface: '#0D0F12',
      surfaceDim: '#080A0C',
      surfaceBright: '#1A1E24',
      surfaceContainerLowest: '#090B0E',
      surfaceContainerLow: '#121519',
      surfaceContainer: '#161B22',
      surfaceContainerHigh: '#1E242C',
      surfaceContainerHighest: '#262D37',
      outline: '#4A3B32',
      outlineVariant: '#2D231E',
      glow: 'rgba(255, 122, 0, 0.15)',
    },
    light: {
      id: 'amber',
      name: 'Blaze Flame',
      primary: '#914C00',
      primaryLight: '#B76300',
      primaryDark: '#6E3A00',
      onPrimary: '#FFFFFF',
      primaryContainer: '#FFDCC1',
      onPrimaryContainer: '#301400',
      secondary: '#745943',
      secondaryContainer: '#FFDCC1',
      onSecondaryContainer: '#2B1706',
      tertiary: '#5D612D',
      tertiaryContainer: '#E2E6A7',
      surface: '#FFF8F5',
      surfaceDim: '#E6DED8',
      surfaceBright: '#FFF8F5',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#FAF2EC',
      surfaceContainer: '#F4ECE5',
      surfaceContainerHigh: '#EEE6DF',
      surfaceContainerHighest: '#E8E1DA',
      outline: '#83746A',
      outlineVariant: '#D6C3B6',
      glow: 'rgba(145, 76, 0, 0.2)',
    },
  },
  blue: {
    name: 'Fedora Sapphire',
    distroHint: 'Fedora Edition',
    dark: {
      id: 'blue',
      name: 'Fedora Sapphire',
      primary: '#9ECAFF',
      primaryLight: '#D1E4FF',
      primaryDark: '#2B75D6',
      onPrimary: '#003258',
      primaryContainer: '#00497D',
      onPrimaryContainer: '#D1E4FF',
      secondary: '#BBC7DB',
      secondaryContainer: '#3B4858',
      onSecondaryContainer: '#D7E3F7',
      tertiary: '#D7BEE4',
      tertiaryContainer: '#533D60',
      surface: '#111418',
      surfaceDim: '#0C0E12',
      surfaceBright: '#37393E',
      surfaceContainerLowest: '#0A0C0F',
      surfaceContainerLow: '#191C20',
      surfaceContainer: '#1D2024',
      surfaceContainerHigh: '#282A2F',
      surfaceContainerHighest: '#33353A',
      outline: '#8B919A',
      outlineVariant: '#42474E',
      glow: 'rgba(43, 117, 214, 0.35)',
    },
    light: {
      id: 'blue',
      name: 'Fedora Sapphire',
      primary: '#0B61A4',
      primaryLight: '#2B75D6',
      primaryDark: '#00497D',
      onPrimary: '#FFFFFF',
      primaryContainer: '#D1E4FF',
      onPrimaryContainer: '#001D36',
      secondary: '#535F70',
      secondaryContainer: '#D7E3F7',
      onSecondaryContainer: '#101C2B',
      tertiary: '#6B5778',
      tertiaryContainer: '#F2DAFF',
      surface: '#F8F9FF',
      surfaceDim: '#DADEE5',
      surfaceBright: '#F8F9FF',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#F2F3FA',
      surfaceContainer: '#ECEEF5',
      surfaceContainerHigh: '#E6E8EF',
      surfaceContainerHighest: '#E1E2E9',
      outline: '#72777F',
      outlineVariant: '#C2C7CF',
      glow: 'rgba(11, 97, 164, 0.2)',
    },
  },
  crimson: {
    name: 'Debian Ruby',
    distroHint: 'Debian Edition',
    dark: {
      id: 'crimson',
      name: 'Debian Ruby',
      primary: '#FFB2BE',
      primaryLight: '#FFD9DF',
      primaryDark: '#D70A53',
      onPrimary: '#660023',
      primaryContainer: '#8F0035',
      onPrimaryContainer: '#FFD9DF',
      secondary: '#E4BDC3',
      secondaryContainer: '#5B3F45',
      onSecondaryContainer: '#FFD9DF',
      tertiary: '#EABF91',
      tertiaryContainer: '#5E411B',
      surface: '#191113',
      surfaceDim: '#130B0D',
      surfaceBright: '#3B3133',
      surfaceContainerLowest: '#100709',
      surfaceContainerLow: '#20181A',
      surfaceContainer: '#251C1E',
      surfaceContainerHigh: '#302628',
      surfaceContainerHighest: '#3C3133',
      outline: '#A08C90',
      outlineVariant: '#534346',
      glow: 'rgba(215, 10, 83, 0.35)',
    },
    light: {
      id: 'crimson',
      name: 'Debian Ruby',
      primary: '#A80040',
      primaryLight: '#D70A53',
      primaryDark: '#800030',
      onPrimary: '#FFFFFF',
      primaryContainer: '#FFD9DF',
      onPrimaryContainer: '#3F0013',
      secondary: '#75565B',
      secondaryContainer: '#FFD9DF',
      onSecondaryContainer: '#2B151A',
      tertiary: '#775730',
      tertiaryContainer: '#FFDDB9',
      surface: '#FFF8F8',
      surfaceDim: '#E7DFE0',
      surfaceBright: '#FFF8F8',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#FAF2F3',
      surfaceContainer: '#F4EBEC',
      surfaceContainerHigh: '#EEE5E7',
      surfaceContainerHighest: '#E8DFE1',
      outline: '#857376',
      outlineVariant: '#D7C1C4',
      glow: 'rgba(168, 0, 64, 0.2)',
    },
  },
  emerald: {
    name: 'Nordic Pine',
    dark: {
      id: 'emerald',
      name: 'Nordic Pine',
      primary: '#85D7A9',
      primaryLight: '#A8F3C5',
      primaryDark: '#00A859',
      onPrimary: '#00381F',
      primaryContainer: '#005230',
      onPrimaryContainer: '#A8F3C5',
      secondary: '#B5CCB9',
      secondaryContainer: '#374B3D',
      onSecondaryContainer: '#D1E9D4',
      tertiary: '#A4CDDC',
      tertiaryContainer: '#244955',
      surface: '#0F1511',
      surfaceDim: '#090F0C',
      surfaceBright: '#343B36',
      surfaceContainerLowest: '#070D0A',
      surfaceContainerLow: '#171D19',
      surfaceContainer: '#1B221D',
      surfaceContainerHigh: '#262D27',
      surfaceContainerHighest: '#313832',
      outline: '#89938B',
      outlineVariant: '#404942',
      glow: 'rgba(0, 168, 89, 0.35)',
    },
    light: {
      id: 'emerald',
      name: 'Nordic Pine',
      primary: '#096C41',
      primaryLight: '#188E58',
      primaryDark: '#005230',
      onPrimary: '#FFFFFF',
      primaryContainer: '#A8F3C5',
      onPrimaryContainer: '#002111',
      secondary: '#4F6354',
      secondaryContainer: '#D1E9D4',
      onSecondaryContainer: '#0C1F13',
      tertiary: '#3C6472',
      tertiaryContainer: '#C0E9F9',
      surface: '#F6FBF5',
      surfaceDim: '#D7DDD7',
      surfaceBright: '#F6FBF5',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#F0F5EF',
      surfaceContainer: '#EAEFE9',
      surfaceContainerHigh: '#E4EAE4',
      surfaceContainerHighest: '#DEE4DE',
      outline: '#707971',
      outlineVariant: '#BFC9BF',
      glow: 'rgba(9, 108, 65, 0.2)',
    },
  },
  purple: {
    name: 'Cosmic Violet',
    dark: {
      id: 'purple',
      name: 'Cosmic Violet',
      primary: '#D3BBFF',
      primaryLight: '#E8DEFF',
      primaryDark: '#8A4FFF',
      onPrimary: '#3F008D',
      primaryContainer: '#5918B8',
      onPrimaryContainer: '#E8DEFF',
      secondary: '#CBC2DB',
      secondaryContainer: '#494458',
      onSecondaryContainer: '#E8DEF8',
      tertiary: '#EFB8C8',
      tertiaryContainer: '#633B48',
      surface: '#141218',
      surfaceDim: '#0E0D12',
      surfaceBright: '#3A383F',
      surfaceContainerLowest: '#0A080E',
      surfaceContainerLow: '#1D1A22',
      surfaceContainer: '#211E26',
      surfaceContainerHigh: '#2C2831',
      surfaceContainerHighest: '#37333C',
      outline: '#948F9A',
      outlineVariant: '#49454E',
      glow: 'rgba(138, 79, 255, 0.35)',
    },
    light: {
      id: 'purple',
      name: 'Cosmic Violet',
      primary: '#6F34CE',
      primaryLight: '#8A4FFF',
      primaryDark: '#5619B5',
      onPrimary: '#FFFFFF',
      primaryContainer: '#E8DEFF',
      onPrimaryContainer: '#24005B',
      secondary: '#615B71',
      secondaryContainer: '#E8DEF8',
      onSecondaryContainer: '#1D192B',
      tertiary: '#7D5260',
      tertiaryContainer: '#FFD8E4',
      surface: '#FEF7FF',
      surfaceDim: '#DFD8DF',
      surfaceBright: '#FEF7FF',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#F8F1F9',
      surfaceContainer: '#F2EBF4',
      surfaceContainerHigh: '#ECE5EE',
      surfaceContainerHighest: '#E7DFE8',
      outline: '#7B7580',
      outlineVariant: '#CBC4CF',
      glow: 'rgba(111, 52, 206, 0.2)',
    },
  },
  neutral: {
    name: 'Cyber Graphite',
    dark: {
      id: 'neutral',
      name: 'Cyber Graphite',
      primary: '#E2E2E6',
      primaryLight: '#FFFFFF',
      primaryDark: '#9E9EA4',
      onPrimary: '#1B1B1F',
      primaryContainer: '#45464A',
      onPrimaryContainer: '#E2E2E6',
      secondary: '#C6C6CA',
      secondaryContainer: '#43474B',
      onSecondaryContainer: '#E1E3E8',
      tertiary: '#C2C7CF',
      tertiaryContainer: '#3C4046',
      surface: '#111114',
      surfaceDim: '#0C0C0E',
      surfaceBright: '#38383C',
      surfaceContainerLowest: '#09090B',
      surfaceContainerLow: '#1A1A1D',
      surfaceContainer: '#1E1E22',
      surfaceContainerHigh: '#29292D',
      surfaceContainerHighest: '#343438',
      outline: '#8E9094',
      outlineVariant: '#44474B',
      glow: 'rgba(226, 226, 230, 0.25)',
    },
    light: {
      id: 'neutral',
      name: 'Cyber Graphite',
      primary: '#47474B',
      primaryLight: '#5F5F63',
      primaryDark: '#2F2F33',
      onPrimary: '#FFFFFF',
      primaryContainer: '#E2E2E6',
      onPrimaryContainer: '#17171A',
      secondary: '#5C5F62',
      secondaryContainer: '#E1E3E8',
      onSecondaryContainer: '#181C1F',
      tertiary: '#585C62',
      tertiaryContainer: '#DDE1E8',
      surface: '#FDFCFE',
      surfaceDim: '#DBDCE0',
      surfaceBright: '#FDFCFE',
      surfaceContainerLowest: '#FFFFFF',
      surfaceContainerLow: '#F4F4F7',
      surfaceContainer: '#EEEEF2',
      surfaceContainerHigh: '#E8E8EC',
      surfaceContainerHighest: '#E2E2E6',
      outline: '#76777B',
      outlineVariant: '#C7C6CA',
      glow: 'rgba(71, 71, 75, 0.15)',
    },
  },
};

interface ThemeContextType {
  colorScheme: M3ThemeColor;
  setColorScheme: (scheme: M3ThemeColor) => void;
  mode: ThemeMode;
  setMode: (mode: ThemeMode) => void;
  distro: DistroBase;
  setDistro: (distro: DistroBase) => void;
  colors: ThemeColors;
  isDark: boolean;
}

const ThemeContext = createContext<ThemeContextType | null>(null);

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [colorScheme, setColorScheme] = useState<M3ThemeColor>(() => (localStorage.getItem('blazehouse-color') as M3ThemeColor) || 'kineticGlass');
  const [mode, setMode] = useState<ThemeMode>(() => (localStorage.getItem('blazehouse-mode') as ThemeMode) || 'dark');
  const [distro, setDistro] = useState<DistroBase>('fedora');

  // Automatically detect the running Linux distribution on launch
  useEffect(() => {
    fetchSystemHardware('fedora').then((hw) => {
      if (hw && hw.distroId) {
        setDistro(hw.distroId as DistroBase);
      }
    });
  }, []);

  // Allow user to switch distro view without overriding their chosen theme
  const handleSetDistro = (newDistro: DistroBase) => {
    setDistro(newDistro);
  };

  const isDark = mode === 'dark' || mode === 'oled';
  const paletteSet = THEME_PALETTES[colorScheme] || THEME_PALETTES.kineticGlass;
  const colors: ThemeColors = isDark ? paletteSet.dark : paletteSet.light;

  // If OLED mode, deep black surface
  const activeColors: ThemeColors = mode === 'oled'
    ? {
        ...colors,
        surface: '#000000',
        surfaceDim: '#000000',
        surfaceContainerLowest: '#000000',
        surfaceContainerLow: '#0A0A0C',
        surfaceContainer: '#121216',
        surfaceContainerHigh: '#1A1A20',
        surfaceContainerHighest: '#24242C',
      }
    : colors;

  useEffect(() => {
    // Apply CSS variables for smooth transitions
    const root = document.documentElement;
    root.style.setProperty('--m3-primary', activeColors.primary);
    root.style.setProperty('--m3-on-primary', activeColors.onPrimary);
    root.style.setProperty('--m3-primary-container', activeColors.primaryContainer);
    root.style.setProperty('--m3-on-primary-container', activeColors.onPrimaryContainer);
    root.style.setProperty('--m3-surface', activeColors.surface);
    root.style.setProperty('--m3-surface-container', activeColors.surfaceContainer);
    root.style.setProperty('--m3-surface-container-high', activeColors.surfaceContainerHigh);
    root.style.setProperty('--m3-surface-container-highest', activeColors.surfaceContainerHighest);
    root.style.setProperty('--m3-outline', activeColors.outline);
    root.style.setProperty('--m3-outline-variant', activeColors.outlineVariant);
    root.style.fontSize = `${Number(localStorage.getItem('blazehouse-font-scale')) || 100}%`;

    if (isDark) {
      document.body.classList.add('dark');
      document.body.classList.remove('light');
    } else {
      document.body.classList.add('light');
      document.body.classList.remove('dark');
    }
    localStorage.setItem('blazehouse-color', colorScheme);
    localStorage.setItem('blazehouse-mode', mode);
  }, [activeColors, isDark, colorScheme, mode]);

  return (
    <ThemeContext.Provider
      value={{
        colorScheme,
        setColorScheme,
        mode,
        setMode,
        distro,
        setDistro: handleSetDistro,
        colors: activeColors,
        isDark,
      }}
    >
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
};
