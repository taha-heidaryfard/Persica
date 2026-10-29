import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/router';
import { Search, X, ArrowRight, Hash } from 'lucide-react';

const SEARCH_INDEX = [
  { title: 'Installation Guide', category: 'Getting Started', path: '/docs/installation', keywords: 'windows installer iscc path cargo setup binary download' },
  { title: 'First Program (Quickstart)', category: 'Getting Started', path: '/docs/quickstart', keywords: 'hello world run script single file zero ceremony' },
  { title: 'persica.toml Configuration', category: 'Project System', path: '/docs/project-system', keywords: 'project toml package entry source persi_env env_dir' },
  { title: 'Isolated .persi_env Environment', category: 'Project System', path: '/docs/project-system', keywords: 'persi_env virtualenv venv isolated packages modules' },
  { title: 'Variables and Static Types', category: 'Core Language', path: '/docs/syntax-basics', keywords: 'int float string bool void final immutability types' },
  { title: 'Control Flow (if, while, for in)', category: 'Core Language', path: '/docs/control-flow', keywords: 'if else while for in range break continue loop' },
  { title: 'Functions and Recursion', category: 'Core Language', path: '/docs/functions', keywords: 'fn return typed recursion forward call void' },
  { title: 'Arrays, Maps & Dictionaries', category: 'Core Language', path: '/docs/collections', keywords: 'array map dict dictionary len append sort slice keys values' },
  { title: 'Classes and Structs (OOP)', category: 'OOP', path: '/docs/oop', keywords: 'struct class this methods constructor value semantics reference' },
  { title: 'Exception Handling (try/catch/throw)', category: 'Robustness', path: '/docs/exceptions', keywords: 'try catch throw exception runtime error unwinding' },
  { title: 'Pythonic Modules & Imports', category: 'Modules', path: '/docs/modules', keywords: 'import from as wildcard circular mod.persi packages' },
  { title: 'Standard Library & Prelude', category: 'Stdlib', path: '/docs/stdlib', keywords: 'Stack Queue Set StringBuilder Pair Result Option math collections' },
  { title: 'CLI Commands Reference', category: 'Tooling', path: '/docs/cli', keywords: 'persica new init run test check eval tokens ast' },
  { title: 'VS Code Extension', category: 'Tooling', path: '/docs/vscode', keywords: 'vscode extension intellisense hover diagnostics snippets vsix' },
  { title: 'راهنمای فارسی زبان پرسیکا', category: 'Persian', path: '/fa', keywords: 'فارسی داکیومنت مستندات آموزش پرسیکا زبان نصب برنامه' },
];

export default function SearchModal({ isOpen, onClose }) {
  const [query, setQuery] = useState('');
  const router = useRouter();

  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        onClose(); // toggle
      }
      if (e.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  if (!isOpen) return null;

  const results = query.trim()
    ? SEARCH_INDEX.filter(item => 
        item.title.toLowerCase().includes(query.toLowerCase()) ||
        item.keywords.toLowerCase().includes(query.toLowerCase())
      )
    : SEARCH_INDEX.slice(0, 7);

  const handleSelect = (path) => {
    onClose();
    router.push(path);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-20 px-4 bg-slate-950/80 backdrop-blur-sm animate-in fade-in">
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
            placeholder="Type a command or keyword to search..."
            className="w-full bg-transparent px-4 py-3.5 text-sm text-slate-200 placeholder:text-slate-500 focus:outline-none"
            autoFocus
          />
          <button
            onClick={onClose}
            className="p-1 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800"
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
                className="w-full flex items-center justify-between px-3.5 py-2.5 rounded-xl text-left text-xs text-slate-300 hover:text-white hover:bg-slate-800 transition group"
              >
                <div className="flex items-center gap-2.5">
                  <Hash className="w-4 h-4 text-emerald-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <div className="font-semibold">{item.title}</div>
                    <div className="text-[10px] text-slate-400">{item.category}</div>
                  </div>
                </div>
                <ArrowRight className="w-3.5 h-3.5 text-slate-400 group-hover:text-emerald-400 transition" />
              </button>
            ))
          ) : (
            <div className="p-6 text-center text-xs text-slate-400">
              No results found for "{query}". Try searching for <span className="text-emerald-400">try</span>, <span className="text-emerald-400">class</span>, or <span className="text-emerald-400">persica.toml</span>.
            </div>
          )}
        </div>

        <div className="px-4 py-2 border-t border-slate-800/80 bg-slate-950/40 flex items-center justify-between text-[11px] text-slate-400">
          <span>Navigate with mouse or keyboard</span>
          <span>ESC to close</span>
        </div>
      </div>
    </div>
  );
}
