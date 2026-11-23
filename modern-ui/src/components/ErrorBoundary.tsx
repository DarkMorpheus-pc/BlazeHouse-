import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';

interface Props {
  children: ReactNode;
  fallbackTitle?: string;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('BlazeHouse UI ErrorBoundary yakaladı:', error, errorInfo);
  }

  private handleReset = () => {
    this.setState({ hasError: false, error: null });
  };

  public render() {
    if (this.state.hasError) {
      return (
        <div className="w-full max-w-xl mx-auto p-6 my-12 rounded-3xl border border-red-500/30 bg-neutral-900 text-red-200 flex flex-col items-center text-center gap-4 shadow-2xl select-none">
          <div className="w-12 h-12 rounded-2xl bg-red-500/20 flex items-center justify-center text-red-400">
            <AlertTriangle className="w-6 h-6" />
          </div>
          <div className="flex flex-col gap-1">
            <h3 className="text-lg font-bold text-white tracking-tight">
              {this.props.fallbackTitle || 'Görünüm Yüklenirken Bir Hata Oluştu'}
            </h3>
            <p className="text-xs text-red-300/80 font-mono max-w-md break-words">
              {this.state.error?.message || 'Beklenmeyen bir arayüz hatası meydana geldi.'}
            </p>
          </div>
          <button
            onClick={this.handleReset}
            className="flex items-center gap-2 px-5 py-2 rounded-full bg-red-500/30 hover:bg-red-500/50 text-white font-medium text-xs transition-colors active:scale-95"
          >
            <RefreshCw className="w-3.5 h-3.5" />
            Tekrar Dene
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}
