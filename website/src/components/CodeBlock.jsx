import React, { useState } from 'react';
import { Check, Copy, Terminal } from 'lucide-react';

export default function CodeBlock({ code, language = 'persica', filename = '', output = '' }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  // Basic syntax highlighter helper for Persica
  const highlightPersica = (text) => {
    const lines = text.split('\n');
    return lines.map((line, idx) => {
      // Escape HTML entities
      let formatted = line
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');

      // Comments
      if (formatted.trim().startsWith('//')) {
        return <div key={idx} className="token-cmt" dangerouslySetInnerHTML={{ __html: formatted }} />;
      }

      // Strings
      formatted = formatted.replace(/"([^"]*)"/g, '<span class="token-str">"$1"</span>');

      // Keywords
      const keywords = ['def', 'class', 'struct', 'this', 'final', 'fn', 'return', 'if', 'else', 'while', 'for', 'in', 'break', 'continue', 'try', 'catch', 'throw', 'import', 'from', 'as'];
      keywords.forEach(kw => {
        const regex = new RegExp(`\\b(${kw})\\b`, 'g');
        formatted = formatted.replace(regex, '<span class="token-kw">$1</span>');
      });

      // Types
      const types = ['int', 'float', 'string', 'bool', 'void', 'any', 'map', 'dict', 'Stack', 'Queue', 'Set', 'StringBuilder', 'Pair', 'Result', 'Option'];
      types.forEach(typ => {
        const regex = new RegExp(`\\b(${typ})\\b`, 'g');
        formatted = formatted.replace(regex, '<span class="token-typ">$1</span>');
      });

      // Builtin functions
      const fns = ['print', 'input', 'read_line', 'len', 'range', 'append', 'to_string', 'to_int', 'to_float', 'abs', 'sqrt', 'pow', 'min', 'max', 'clamp', 'sort', 'reverse', 'slice', 'sum', 'keys', 'values', 'contains_key'];
      fns.forEach(fn => {
        const regex = new RegExp(`\\b(${fn})\\b(?=\\()`, 'g');
        formatted = formatted.replace(regex, '<span class="token-fn">$1</span>');
      });

      // Numbers
      formatted = formatted.replace(/\b(\d+(\.\d+)?)\b/g, '<span class="token-num">$1</span>');

      return <div key={idx} dangerouslySetInnerHTML={{ __html: formatted }} />;
    });
  };

  return (
    <div className="my-5 rounded-xl border border-slate-800 bg-slate-950/90 shadow-2xl overflow-hidden">
      {filename && (
        <div className="flex items-center justify-between px-4 py-2 border-b border-slate-800/80 bg-slate-900/60 text-xs font-mono text-slate-400">
          <span className="flex items-center gap-1.5 font-medium text-slate-300">
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-500/30 border border-emerald-500/60 inline-block" />
            {filename}
          </span>
          <button
            onClick={handleCopy}
            className="flex items-center gap-1 px-2 py-1 rounded bg-slate-800/80 hover:bg-slate-700/80 text-slate-300 hover:text-white transition-all text-xs"
            title="Copy code"
          >
            {copied ? (
              <>
                <Check className="w-3.5 h-3.5 text-emerald-400" />
                <span className="text-emerald-400 font-medium">Copied!</span>
              </>
            ) : (
              <>
                <Copy className="w-3.5 h-3.5" />
                <span>Copy</span>
              </>
            )}
          </button>
        </div>
      )}

      <div className="relative">
        {!filename && (
          <button
            onClick={handleCopy}
            className="absolute top-3 right-3 z-10 flex items-center gap-1 px-2 py-1 rounded bg-slate-800/90 hover:bg-slate-700 text-slate-300 hover:text-white transition-all text-xs border border-slate-700/60"
            title="Copy code"
          >
            {copied ? (
              <>
                <Check className="w-3.5 h-3.5 text-emerald-400" />
                <span className="text-emerald-400 font-medium">Copied!</span>
              </>
            ) : (
              <>
                <Copy className="w-3.5 h-3.5" />
                <span>Copy</span>
              </>
            )}
          </button>
        )}
        <pre className="p-4 font-mono text-[13.5px] leading-relaxed overflow-x-auto text-slate-100 select-text">
          <code>{language === 'persica' ? highlightPersica(code) : code}</code>
        </pre>
      </div>

      {output && (
        <div className="border-t border-slate-800/90 bg-slate-900/90 px-4 py-3">
          <div className="flex items-center gap-1.5 text-xs font-mono text-slate-400 mb-1.5 font-semibold">
            <Terminal className="w-3.5 h-3.5 text-emerald-400" />
            <span>Output:</span>
          </div>
          <pre className="font-mono text-xs text-emerald-300/90 whitespace-pre-wrap leading-snug">
            {output}
          </pre>
        </div>
      )}
    </div>
  );
}
