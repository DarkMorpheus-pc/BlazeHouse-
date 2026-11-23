import React from 'react';
import { SystemControlsView } from './SystemControlsView';
export const PerformanceGamingView = (props: Omit<React.ComponentProps<typeof SystemControlsView>, 'group'>) => <SystemControlsView {...props} group="gaming" />;
