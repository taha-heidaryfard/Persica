import React, { useState } from 'react';
import { Check, Copy, Terminal } from 'lucide-react';

const KEYWORDS = new Set([
  'def', 'class', 'struct', 'this', 'final', 'fn', 'return',
  'if', 'else', 'while', 'for', 'in', 'break', 'continue',
  'try', 'catch', 'throw', 'import', 'from', 'as'
]);

const TYPES = new Set([
  'int', 'float', 'string', 'bool', 'void', 'any', 'map', 'dict',
  'Stack', 'Queue', 'Set', 'StringBuilder', 'Pair', 'Result', 'Option'
]);

const BUILTINS = new Set([
  'print', 'input', 'read_line', 'len', 'range', 'append',
  'to_string', 'to_int', 'to_float', 'abs', 'sqrt', 'pow',
  'min', 'max', 'clamp', 'sort', 'reverse', 'slice', 'sum',
  'keys', 'values', 'contains_key'
]);

// Tokenizer regex that cleanly segments Persica code into tokens without HTML tag corruption
const TOKEN_REGEX = /(\/\/[^\n]*)|("(?:[^"\\]|\\.)*")|(\b\d+(?:\.\d+)?\b)|([a-zA-Z_][a-zA-Z0-9_]*)|([^a-zA-Z0-9_"\s/]+|\s+|\/)/g;

function highlightPersica(text) {
  if (!text) return null;
  const lines = text.split('\n');
  return lines.map((line, lineIdx) => {
    // Entire line is a comment
    if (line.trim().startsWith('//')) {
      return (
        <div key={lineIdx} className="token-cmt">
          {line || ' '}
        </div>
      );
    }

    const tokens = [];
    let match;
    let tokenKey = 0;
    TOKEN_REGEX.lastIndex = 0;

    while ((match = TOKEN_REGEX.exec(line)) !== null) {
      const [full, cmt, str, num, word, other] = match;

      if (cmt !== undefined) {
        tokens.push(
          <span key={tokenKey++} className="token-cmt">
            {cmt}
          </span>
        );
      } else if (str !== undefined) {
        tokens.push(
          <span key={tokenKey++} className="token-str">
            {str}
          </span>
        );
      } else if (num !== undefined) {
        tokens.push(
          <span key={tokenKey++} className="token-num">
            {num}
          </span>
        );
      } else if (word !== undefined) {
        if (KEYWORDS.has(word)) {
          tokens.push(
            <span key={tokenKey++} className="token-kw">
              {word}
            </span>
          );
        } else if (TYPES.has(word)) {
          tokens.push(
            <span key={tokenKey++} className="token-typ">
              {word}
            </span>
          );
        } else if (BUILTINS.has(word)) {
          tokens.push(
            <span key={tokenKey++} className="token-fn">
              {word}
            </span>
          );
        } else {
          tokens.push(<span key={tokenKey++}>{word}</span>);
        }
      } else if (other !== undefined) {
        tokens.push(<span key={tokenKey++}>{other}</span>);
      }
    }

    return (
      <div key={lineIdx}>
        {tokens.length > 0 ? tokens : ' '}
      </div>
    );
  });
}

export default function CodeBlock({ code, language = 'persica', filename = '', output = '' }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
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
