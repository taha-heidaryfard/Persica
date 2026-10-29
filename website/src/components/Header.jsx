import React from 'react';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { Heart, Search, Github, BookOpen, Terminal, Code2, Menu, X, Globe } from 'lucide-react';

export default function Header({ onOpenSearch, onToggleMobileNav, isMobileNavOpen }) {
  const router = useRouter();
  const isFa = router.pathname.startsWith('/fa');

  // Compute opposite language path
  let targetLangPath = '/';
  if (isFa) {
    // Current is Persian, switch to English
    targetLangPath = router.pathname.replace(/^\/fa/, '') || '/';
  } else {
    // Current is English, switch to Persian
    targetLangPath = router.pathname === '/' ? '/fa' : `/fa${router.pathname}`;
  }

  return (
    <header className="sticky top-0 z-40 h-16 w-full border-b border-slate-800/80 bg-slate-950/85 backdrop-blur-xl transition-colors">
      <div className="mx-auto flex h-full max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        
        {/* Left: Mobile Nav Toggle & Brand */}
        <div className="flex items-center gap-3">
          <button
            onClick={onToggleMobileNav}
            className="md:hidden flex items-center justify-center p-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition"
            aria-label="Toggle Navigation"
          >
            {isMobileNavOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
          </button>

          <Link href={isFa ? "/fa" : "/"} className="flex items-center gap-2.5 group">
            <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-tr from-emerald-600 to-teal-400 text-white shadow-lg shadow-emerald-500/20 group-hover:scale-105 transition-transform">
              <span className="text-lg">🌿</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-lg font-bold tracking-tight text-white group-hover:text-emerald-400 transition-colors">
                {isFa ? "پرسیکا" : "Persica"}
              </span>
              <span className="rounded-full border border-emerald-500/30 bg-emerald-500/10 px-2 py-0.5 text-[11px] font-semibold text-emerald-400 font-sans">
                v0.13.0
              </span>
            </div>
          </Link>
        </div>

        {/* Center: Search Bar Trigger */}
        <div className="hidden sm:flex flex-1 max-w-md mx-6">
          <button
            onClick={onOpenSearch}
            className="flex w-full items-center justify-between rounded-xl border border-slate-800 bg-slate-900/60 px-3.5 py-1.5 text-xs text-slate-400 hover:border-slate-700 hover:text-slate-300 transition-all shadow-inner"
          >
            <span className="flex items-center gap-2">
              <Search className="h-3.5 w-3.5 text-slate-400" />
              <span>{isFa ? "جستجو در مستندات، کلمات کلیدی، توابع..." : "Search docs, keywords, built-ins..."}</span>
            </span>
            <kbd className="hidden sm:inline-block rounded border border-slate-700 bg-slate-800/80 px-1.5 py-0.5 text-[10px] font-mono text-slate-400">
              Ctrl K
            </kbd>
          </button>
        </div>

        {/* Right: Navigation Links & Actions */}
        <div className="flex items-center gap-2.5">
          <nav className="hidden lg:flex items-center gap-1 text-sm font-medium text-slate-300">
            <Link
              href={isFa ? "/fa/docs/installation" : "/docs/installation"}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg hover:text-white hover:bg-slate-800/60 transition ${
                router.pathname.includes('/docs/installation') ? 'text-emerald-400 font-semibold' : ''
              }`}
            >
              <BookOpen className="w-4 h-4" />
              <span>{isFa ? "مستندات" : "Docs"}</span>
            </Link>
            <Link
              href={isFa ? "/fa/docs/cli" : "/docs/cli"}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg hover:text-white hover:bg-slate-800/60 transition"
            >
              <Terminal className="w-4 h-4" />
              <span>{isFa ? "خط فرمان" : "CLI"}</span>
            </Link>
            <Link
              href={isFa ? "/fa/docs/vscode" : "/docs/vscode"}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg hover:text-white hover:bg-slate-800/60 transition"
            >
              <Code2 className="w-4 h-4" />
              <span>VS Code</span>
            </Link>
          </nav>

          <div className="h-4 w-px bg-slate-800 hidden lg:block" />

          {/* Language Switcher Button */}
          <Link
            href={targetLangPath}
            className="flex items-center gap-1.5 rounded-lg border border-slate-800 bg-slate-900/80 px-2.5 py-1.5 text-xs font-semibold text-slate-200 hover:border-emerald-500/50 hover:text-emerald-400 transition"
            title={isFa ? "Switch to English" : "تغییر به فارسی"}
          >
            <Globe className="w-3.5 h-3.5 text-emerald-400" />
            <span className={isFa ? "font-sans" : "font-vazir"}>{isFa ? "English" : "فارسی"}</span>
          </Link>

          {/* Reymit Sponsor Button */}
          <a
            href="https://reymit.ir/taxtander"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 rounded-lg bg-gradient-to-r from-rose-500 to-pink-600 px-3 py-1.5 text-xs font-semibold text-white shadow-md shadow-rose-500/20 hover:from-rose-600 hover:to-pink-700 hover:shadow-rose-500/35 transition-all transform hover:-translate-y-0.5"
          >
            <Heart className="h-3.5 w-3.5 fill-white animate-pulse" />
            <span className="hidden sm:inline">{isFa ? "حمایت (Reymit)" : "Sponsor (Reymit)"}</span>
            <span className="sm:hidden">حمایت</span>
          </a>

          {/* GitHub Repo */}
          <a
            href="https://github.com/taha-heidaryfard/Persica"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center justify-center p-2 rounded-lg border border-slate-800 bg-slate-900/60 text-slate-300 hover:text-white hover:border-slate-700 transition"
            title="GitHub Repository"
          >
            <Github className="w-4 h-4" />
          </a>
        </div>

      </div>
    </header>
  );
}
