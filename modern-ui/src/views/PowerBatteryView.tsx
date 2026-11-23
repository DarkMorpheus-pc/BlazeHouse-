import React from 'react';
import { SystemControlsView } from './SystemControlsView';
export const PowerBatteryView = (props: Omit<React.ComponentProps<typeof SystemControlsView>, 'group'>) => <SystemControlsView {...props} group="power" />;
