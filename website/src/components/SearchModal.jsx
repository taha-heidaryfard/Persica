import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/router';
import { Search, X, ArrowRight, ArrowLeft, Hash } from 'lucide-react';

const SEARCH_INDEX_EN = [
  { title: 'Installation & Setup Guide', category: 'Getting Started', path: '/docs/installation', keywords: 'windows installer iscc path cargo setup binary download exe' },
  { title: 'First Program (Quickstart)', category: 'Getting Started', path: '/docs/quickstart', keywords: 'hello world run script single file zero ceremony fibonacci' },
  { title: 'persica.toml & .persi_env Isolation', category: 'Project System', path: '/docs/project-system', keywords: 'project toml package entry source persi_env env_dir virtualenv isolated' },
  { title: 'Variables, Static Types & final', category: 'Core Language', path: '/docs/syntax-basics', keywords: 'int float string bool void final immutability types casting' },
  { title: 'Control Flow (if, while, for in, range)', category: 'Core Language', path: '/docs/control-flow', keywords: 'if else while for in range break continue loop conditions' },
  { title: 'Functions, Signatures & Recursion', category: 'Core Language', path: '/docs/functions', keywords: 'fn return typed recursion forward call void deterministic' },
  { title: 'Arrays, Maps & Dictionaries', category: 'Core Language', path: '/docs/collections', keywords: 'array map dict dictionary len append sort slice keys values multidimensional' },
  { title: 'Classes and Structs (OOP)', category: 'OOP', path: '/docs/oop', keywords: 'struct class this methods constructor value semantics reference heap' },
  { title: 'Exception Handling (try/catch/throw)', category: 'Robustness', path: '/docs/exceptions', keywords: 'try catch throw exception runtime error unwinding panic safety' },
  { title: 'Pythonic Modules & Packages', category: 'Modules', path: '/docs/modules', keywords: 'import from as wildcard circular mod.persi packages folder' },
  { title: 'Standard Library & Auto-Prelude', category: 'Stdlib', path: '/docs/stdlib', keywords: 'Stack Queue Set StringBuilder Pair Result Option math collections self-hosted' },
  { title: 'CLI Commands Reference', category: 'Tooling', path: '/docs/cli', keywords: 'persica new init run test check eval tokens ast format' },
  { title: 'VS Code Extension & IntelliSense', category: 'Tooling', path: '/docs/vscode', keywords: 'vscode extension intellisense hover diagnostics snippets vsix syntax' },
];

const SEARCH_INDEX_FA = [
  { title: 'راهنمای نصب و راه‌اندازی ویندوز', category: 'شروع کار', path: '/fa/docs/installation', keywords: 'نصب راه اندازی ویندوز ستاپ اینستالر پث مسیر کارگو دانلود setup installer path cargo' },
  { title: 'اولین برنامه و شروع سریع', category: 'شروع کار', path: '/fa/docs/quickstart', keywords: 'شروع سریع برنامه اجرا اسکریپت تکی فیبوناچی پرینت quickstart run' },
  { title: 'پیکربندی persica.toml و محیط .persi_env', category: 'معماری پروژه', path: '/fa/docs/project-system', keywords: 'پروژه تامل پکیج محیط ایزوله ماژول کتابخانه persica.toml persi_env' },
  { title: 'متغیرها، انواع داده ایستا و final', category: 'هسته زبان', path: '/fa/docs/syntax-basics', keywords: 'متغیر دیتا تایپ انواع داده ایستا تغییرناپذیر کستینگ types int string bool' },
  { title: 'شروط و حلقه‌ها (if, while, for in)', category: 'هسته زبان', path: '/fa/docs/control-flow', keywords: 'شرط حلقه فور وایل رنج تکرار break continue loop if for in range' },
  { title: 'توابع، امضا و فراخوانی بازگشتی', category: 'هسته زبان', path: '/fa/docs/functions', keywords: 'تابع توابع بازگشتی آرگومان خروجی بازگشت فانکشن fn return' },
  { title: 'آرایه‌ها، دیکشنری و مپ‌ها', category: 'هسته زبان', path: '/fa/docs/collections', keywords: 'آرایه مپ دیکشنری لیست مجموعه کلید مقدار array map dict' },
  { title: 'شیء‌گرایی: مقایسه struct و class', category: 'شیء‌گرایی', path: '/fa/docs/oop', keywords: 'کلاس استراکت متد اینستنس شی گرایی سازنده struct class this oop' },
  { title: 'مدیریت خطا (try / catch / throw)', category: 'پایداری و خطا', path: '/fa/docs/exceptions', keywords: 'خطا استثنا ترای کچ پرتاب خطا مهار خطا try catch throw exception' },
  { title: 'سیستم پکیج و ایمپورت ماژول‌ها', category: 'ماژول‌ها', path: '/fa/docs/modules', keywords: 'ماژول ایمپورت پکیج ماژولار ایمپورت پایتونی mod.persi import from' },
  { title: 'کتابخانه استاندارد خودمیزبان و Prelude', category: 'کتابخانه استاندارد', path: '/fa/docs/stdlib', keywords: 'کتابخانه استاندارد استک صف ست بیلدر پشته پیش‌درآمد stack queue set' },
  { title: 'مرجع جامع دستورات خط فرمان CLI', category: 'ابزارها', path: '/fa/docs/cli', keywords: 'خط فرمان ترمینال دستورات دستور کامند پرچم آرگومان cli run test' },
  { title: 'افزونه رسمی وی‌اس‌کد VS Code', category: 'ابزارها', path: '/fa/docs/vscode', keywords: 'وی اس کد افزونه اکستنشن اینتلیسنس هایلایت سینتکس اسنیپت vscode vsix' },
];

export default function SearchModal({ isOpen, onClose }) {
  const [query, setQuery] = useState('');
  const router = useRouter();
  const isFa = router.pathname.startsWith('/fa');

  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        onClose();
      }
      if (e.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  if (!isOpen) return null;

  const activeIndex = isFa ? [...SEARCH_INDEX_FA, ...SEARCH_INDEX_EN] : [...SEARCH_INDEX_EN, ...SEARCH_INDEX_FA];

  const results = query.trim()
    ? activeIndex.filter(item => 
        item.title.toLowerCase().includes(query.toLowerCase()) ||
        item.keywords.toLowerCase().includes(query.toLowerCase()) ||
        item.category.toLowerCase().includes(query.toLowerCase())
      )
    : activeIndex.slice(0, 7);

  const handleSelect = (path) => {
    onClose();
    router.push(path);
  };

  const ArrowIcon = isFa ? ArrowLeft : ArrowRight;

  return (
    <div 
      className="fixed inset-0 z-50 flex items-start justify-center pt-20 px-4 bg-slate-950/80 backdrop-blur-sm animate-in fade-in"
      dir={isFa ? 'rtl' : 'ltr'}
    >
      <div 
        className="relative w-full max-w-xl rounded-2xl border border-slate-700 bg-slate-900 shadow-2xl overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center px-4 border-b border-slate-800 bg-slate-950/60">
          <Search className="w-5 h-5 text-slate-400 shrink-0" />
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={isFa ? "جستجو در مستندات، توابع، کلمات کلیدی پرسیکا..." : "Type a command or keyword to search..."}
            className={`w-full bg-transparent px-4 py-3.5 text-sm text-slate-200 placeholder:text-slate-500 focus:outline-none ${isFa ? 'font-vazir text-right' : 'font-sans text-left'}`}
            autoFocus
          />
          <button
            onClick={onClose}
            className="p-1 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="max-h-80 overflow-y-auto p-2 space-y-1">
          {results.length > 0 ? (
            results.map((item, idx) => (
              <button
                key={idx}
                onClick={() => handleSelect(item.path)}
                className="w-full flex items-center justify-between px-3.5 py-2.5 rounded-xl text-start text-xs text-slate-300 hover:text-white hover:bg-slate-800 transition group"
              >
                <div className="flex items-center gap-2.5">
                  <Hash className="w-4 h-4 text-emerald-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <div className="font-semibold">{item.title}</div>
                    <div className="text-[10px] text-slate-400">{item.category}</div>
                  </div>
                </div>
                <ArrowIcon className="w-3.5 h-3.5 text-slate-400 group-hover:text-emerald-400 transition" />
              </button>
            ))
          ) : (
            <div className="p-6 text-center text-xs text-slate-400">
              {isFa ? (
                <>نتیجه‌ای برای "{query}" یافت نشد. کلماتی مانند <span className="text-emerald-400">try</span>، <span className="text-emerald-400">class</span>، یا <span className="text-emerald-400">نصب</span> را جستجو کنید.</>
              ) : (
                <>No results found for "{query}". Try searching for <span className="text-emerald-400">try</span>, <span className="text-emerald-400">class</span>, or <span className="text-emerald-400">persica.toml</span>.</>
              )}
            </div>
          )}
        </div>

        <div className="flex items-center justify-between px-4 py-2 border-t border-slate-800 bg-slate-950/40 text-[10px] text-slate-400">
          <span>{isFa ? "برای انتخاب روی گزینه کلیک کنید" : "Navigate with clicks or shortcuts"}</span>
          <span className="font-mono">ESC {isFa ? "برای بستن" : "to close"}</span>
        </div>
      </div>
    </div>
  );
}
