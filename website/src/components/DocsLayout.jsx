import React, { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/router';
import Header from './Header';
import Sidebar, { NAVIGATION_SECTIONS } from './Sidebar';
import SearchModal from './SearchModal';
import { ChevronRight, ArrowLeft, ArrowRight, Heart, Sparkles, Bot } from 'lucide-react';

export default function DocsLayout({ children, meta = {}, toc = [] }) {
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [isMobileNavOpen, setIsMobileNavOpen] = useState(false);
  const router = useRouter();

  // Find previous and next pages
  const allItems = NAVIGATION_SECTIONS.flatMap(s => s.items);
  const currentIndex = allItems.findIndex(i => i.href === router.pathname);
  const prevItem = currentIndex > 0 ? allItems[currentIndex - 1] : null;
  const nextItem = currentIndex < allItems.length - 1 ? allItems[currentIndex + 1] : null;

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-emerald-500/30 selection:text-emerald-200">
      <Header
        onOpenSearch={() => setIsSearchOpen(true)}
        onToggleMobileNav={() => setIsMobileNavOpen(!isMobileNavOpen)}
        isMobileNavOpen={isMobileNavOpen}
      />

      <SearchModal
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
      />

      {/* Main Container */}
      <div className="mx-auto flex w-full max-w-7xl flex-1 px-4 sm:px-6 lg:px-8">
        
        {/* Desktop Left Sidebar */}
        <div className="hidden md:block">
          <Sidebar className="sticky top-16 h-[calc(100vh-4rem)] border-r border-slate-800/80" />
        </div>

        {/* Mobile Navigation Drawer */}
        {isMobileNavOpen && (
          <div className="fixed inset-0 top-16 z-30 bg-slate-950/95 p-6 overflow-y-auto md:hidden backdrop-blur-xl">
            <Sidebar className="w-full" />
          </div>
        )}

        {/* Center Main Content Area */}
        <main className="min-w-0 flex-1 py-8 md:px-8 lg:px-12">
          
          {/* AI Notice Banner */}
          <div className="mb-6 flex items-center justify-between gap-3 rounded-xl border border-purple-500/30 bg-purple-500/10 px-4 py-2.5 text-xs text-purple-300">
            <div className="flex items-center gap-2">
              <Bot className="w-4 h-4 text-purple-400 shrink-0" />
              <span>This documentation website was built with AI • این وب‌سایت با کمک هوش مصنوعی ساخته شده است</span>
            </div>
            <span className="hidden sm:inline font-mono text-[10px] text-purple-400/80">UI/UX Pro Max</span>
          </div>

          {/* Breadcrumbs */}
          <nav className="mb-4 flex items-center gap-1.5 text-xs font-medium text-slate-400">
            <Link href="/" className="hover:text-slate-300 transition">Persica</Link>
            <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
            <Link href="/docs/installation" className="hover:text-slate-300 transition">Docs</Link>
            {meta.category && (
              <>
                <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
                <span className="text-slate-400">{meta.category}</span>
              </>
            )}
            {meta.title && (
              <>
                <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
                <span className="text-emerald-400 font-semibold">{meta.title}</span>
              </>
            )}
          </nav>

          {/* Title Header */}
          <header className="mb-8 border-b border-slate-800/80 pb-6">
            <div className="flex flex-wrap items-center justify-between gap-3">
              <h1 className="text-3xl font-extrabold tracking-tight text-white sm:text-4xl">
                {meta.title}
              </h1>
              {meta.badge && (
                <span className="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-3 py-1 text-xs font-semibold text-emerald-400">
                  {meta.badge}
                </span>
              )}
            </div>
            {meta.description && (
              <p className="mt-3 text-base text-slate-400 leading-relaxed max-w-3xl">
                {meta.description}
              </p>
            )}
          </header>

          {/* Article Body */}
          <article className="prose prose-invert prose-emerald max-w-none prose-headings:font-bold prose-headings:tracking-tight prose-headings:text-white prose-p:text-slate-300 prose-p:leading-relaxed prose-a:text-emerald-400 hover:prose-a:text-emerald-300 prose-code:text-emerald-300 prose-code:bg-slate-900 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-code:before:content-none prose-code:after:content-none">
            {children}
          </article>

          {/* Reymit Sponsor Card */}
          <div className="my-12 rounded-2xl border border-rose-500/30 bg-gradient-to-r from-rose-950/20 via-pink-950/20 to-slate-900/60 p-6 shadow-xl">
            <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
              <div className="space-y-1">
                <div className="flex items-center gap-2 text-rose-400 font-bold text-sm">
                  <Heart className="w-4 h-4 fill-rose-500 text-rose-500" />
                  <span>حمایت از توسعه زبان برنامه‌نویسی پرسیکا</span>
                </div>
                <p className="text-xs text-slate-400 leading-relaxed">
                  پرسیکا پروژه‌ای مستقل با صدها ساعت پژوهش و مهندسی کامپایلر است. با حمایت مالی در درگاه رِیمیت از Taxtander به ادامه این مسیر کمک کنید.
                </p>
              </div>
              <a
                href="https://reymit.ir/taxtander"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex shrink-0 items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-rose-500 to-pink-600 px-4 py-2.5 text-xs font-bold text-white shadow-lg shadow-rose-500/25 hover:from-rose-600 hover:to-pink-700 transition transform hover:-translate-y-0.5"
              >
                <span>ورود به درگاه رِیمیت</span>
                <ArrowRight className="w-3.5 h-3.5" />
              </a>
            </div>
          </div>

          {/* Prev / Next Navigation Footer */}
          <div className="mt-12 flex flex-col sm:flex-row items-center justify-between gap-4 border-t border-slate-800/80 pt-6">
            {prevItem ? (
              <Link
                href={prevItem.href}
                className="flex w-full sm:w-auto items-center gap-3 rounded-xl border border-slate-800 bg-slate-900/40 p-3.5 text-left transition hover:border-slate-700 hover:bg-slate-900 text-xs"
              >
                <ArrowLeft className="w-4 h-4 text-slate-400" />
                <div>
                  <div className="text-[10px] text-slate-400 font-medium uppercase">Previous</div>
                  <div className="font-semibold text-slate-200">{prevItem.title}</div>
                </div>
              </Link>
            ) : <div />}

            {nextItem && (
              <Link
                href={nextItem.href}
                className="flex w-full sm:w-auto items-center justify-between gap-3 rounded-xl border border-slate-800 bg-slate-900/40 p-3.5 text-right transition hover:border-slate-700 hover:bg-slate-900 text-xs"
              >
                <div>
                  <div className="text-[10px] text-slate-400 font-medium uppercase">Next</div>
                  <div className="font-semibold text-slate-200">{nextItem.title}</div>
                </div>
                <ArrowRight className="w-4 h-4 text-slate-400" />
              </Link>
            )}
          </div>

          {/* Footer Attribution */}
          <footer className="mt-16 border-t border-slate-800/80 pt-8 pb-12 text-center text-xs text-slate-400">
            <p>
              Designed & developed by <strong className="text-slate-300 font-semibold">Mohammad Taha Heidaryfard (Taxtander)</strong> • Under the MIT License
            </p>
            <p className="mt-1 text-[11px] text-slate-400">
              Persica Compiler & Standard Library v0.13.0 • Built completely from scratch in Rust
            </p>
          </footer>
        </main>

        {/* Right Table of Contents (Desktop >= 1280px) */}
        {toc.length > 0 && (
          <aside className="hidden xl:block w-64 shrink-0 py-8 pl-6 border-l border-slate-800/80 text-xs">
            <div className="sticky top-20 space-y-3">
              <h4 className="font-bold uppercase tracking-wider text-slate-400 text-[11px]">
                On This Page
              </h4>
              <nav className="space-y-1.5">
                {toc.map((item, idx) => (
                  <a
                    key={idx}
                    href={`#${item.id}`}
                    className={`block text-slate-400 hover:text-emerald-400 transition truncate ${
                      item.level === 3 ? 'pl-3 text-[11px]' : 'font-medium'
                    }`}
                  >
                    {item.title}
                  </a>
                ))}
              </nav>
            </div>
          </aside>
        )}

      </div>
    </div>
  );
}
