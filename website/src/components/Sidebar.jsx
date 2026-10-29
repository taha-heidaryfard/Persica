import React from 'react';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { 
  Download, Play, FolderTree, FileCode2, GitFork, 
  Layers, Box, ShieldAlert, Cpu, Terminal, 
  Code2, Languages, Sparkles
} from 'lucide-react';

export const NAVIGATION_EN = [
  {
    title: 'Getting Started',
    items: [
      { title: 'Installation Guide', href: '/docs/installation', icon: Download },
      { title: 'First Program & Quickstart', href: '/docs/quickstart', icon: Play },
    ]
  },
  {
    title: 'Project Architecture',
    items: [
      { title: 'persica.toml & .persi_env', href: '/docs/project-system', icon: FolderTree, badge: 'New v0.13' },
    ]
  },
  {
    title: 'Core Language',
    items: [
      { title: 'Variables & Data Types', href: '/docs/syntax-basics', icon: FileCode2 },
      { title: 'Control Flow & Loops', href: '/docs/control-flow', icon: GitFork },
      { title: 'Functions & Recursion', href: '/docs/functions', icon: Code2 },
      { title: 'Arrays, Maps & Dictionaries', href: '/docs/collections', icon: Layers },
    ]
  },
  {
    title: 'Object-Oriented & Robustness',
    items: [
      { title: 'Structs & Classes (OOP)', href: '/docs/oop', icon: Box },
      { title: 'Exception Handling (try/catch)', href: '/docs/exceptions', icon: ShieldAlert },
      { title: 'Pythonic Modules & Imports', href: '/docs/modules', icon: GitFork },
      { title: 'Prelude & Standard Library', href: '/docs/stdlib', icon: Cpu },
    ]
  },
  {
    title: 'Tooling & Ecosystem',
    items: [
      { title: 'CLI Commands Reference', href: '/docs/cli', icon: Terminal },
      { title: 'VS Code Extension', href: '/docs/vscode', icon: Sparkles, badge: 'Official' },
    ]
  }
];

export const NAVIGATION_FA = [
  {
    title: 'شروع کار',
    items: [
      { title: 'راهنمای نصب و راه‌اندازی', href: '/fa/docs/installation', icon: Download },
      { title: 'اولین برنامه و شروع سریع', href: '/fa/docs/quickstart', icon: Play },
    ]
  },
  {
    title: 'معماری پروژه',
    items: [
      { title: 'پیکربندی persica.toml و .persi_env', href: '/fa/docs/project-system', icon: FolderTree, badge: 'نسخه جدید ۰.۱۳' },
    ]
  },
  {
    title: 'هسته زبان',
    items: [
      { title: 'متغیرها و انواع داده', href: '/fa/docs/syntax-basics', icon: FileCode2 },
      { title: 'شروط و حلقه‌ها', href: '/fa/docs/control-flow', icon: GitFork },
      { title: 'توابع و بازگشت', href: '/fa/docs/functions', icon: Code2 },
      { title: 'آرایه‌ها، مپ‌ها و دیکشنری', href: '/fa/docs/collections', icon: Layers },
    ]
  },
  {
    title: 'شیء‌گرایی و پایداری',
    items: [
      { title: 'استراکت‌ها و کلاس‌ها (OOP)', href: '/fa/docs/oop', icon: Box },
      { title: 'مدیریت خطا (try / catch / throw)', href: '/fa/docs/exceptions', icon: ShieldAlert },
      { title: 'سیستم پکیج و ماژول‌ها', href: '/fa/docs/modules', icon: GitFork },
      { title: 'کتابخانه استاندارد و پیش‌درآمد', href: '/fa/docs/stdlib', icon: Cpu },
    ]
  },
  {
    title: 'ابزارها و اکوسیستم',
    items: [
      { title: 'مرجع دستورات خط فرمان CLI', href: '/fa/docs/cli', icon: Terminal },
      { title: 'افزونه رسمی VS Code', href: '/fa/docs/vscode', icon: Sparkles, badge: 'رسمی' },
    ]
  }
];

export default function Sidebar({ className = '' }) {
  const router = useRouter();
  const isFa = router.pathname.startsWith('/fa');
  const sections = isFa ? NAVIGATION_FA : NAVIGATION_EN;

  return (
    <aside className={`w-72 shrink-0 py-6 ${isFa ? 'pl-4' : 'pr-4'} overflow-y-auto ${className}`}>
      <div className="space-y-7">
        {sections.map((section, idx) => (
          <div key={idx} className="space-y-2">
            <h4 className="text-[11px] font-bold uppercase tracking-wider text-slate-400 px-3">
              {section.title}
            </h4>
            <div className="space-y-0.5">
              {section.items.map((item, itemIdx) => {
                const Icon = item.icon;
                const isActive = router.pathname === item.href || (item.href !== '/' && router.asPath.startsWith(item.href));

                return (
                  <Link
                    key={itemIdx}
                    href={item.href}
                    className={`flex items-center justify-between px-3 py-2 rounded-xl text-xs font-medium transition-all group ${
                      isActive
                        ? 'bg-emerald-500/15 text-emerald-400 font-semibold border border-emerald-500/30'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900/80'
                    }`}
                  >
                    <span className="flex items-center gap-2.5 truncate">
                      <Icon className={`w-4 h-4 shrink-0 transition-colors ${
                        isActive ? 'text-emerald-400' : 'text-slate-400 group-hover:text-slate-300'
                      }`} />
                      <span className="truncate">{item.title}</span>
                    </span>
                    {item.badge && (
                      <span className="rounded-md border border-emerald-500/40 bg-emerald-500/10 px-1.5 py-0.5 text-[9px] font-bold text-emerald-400 uppercase tracking-wider shrink-0">
                        {item.badge}
                      </span>
                    )}
                  </Link>
                );
              })}
            </div>
          </div>
        ))}
      </div>
    </aside>
  );
}
