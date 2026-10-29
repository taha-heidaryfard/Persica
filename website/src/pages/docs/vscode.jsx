import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { Sparkles, Download, CheckCircle, Code2, Terminal } from 'lucide-react';

export default function VSCodePage() {
  const meta = {
    title: 'VS Code Extension',
    category: 'Tooling & Ecosystem',
    badge: 'Official v0.13.0',
    description: 'Turn Visual Studio Code into a full-featured IDE for the Persica programming language.',
  };

  const toc = [
    { title: 'Overview & Capabilities', id: 'overview', level: 2 },
    { title: 'Installing the Extension (.vsix)', id: 'installation', level: 2 },
    { title: 'IntelliSense & Auto-Complete', id: 'intellisense', level: 2 },
    { title: 'Hover Documentation', id: 'hover', level: 2 },
    { title: 'Live Error Diagnostics on Save', id: 'diagnostics', level: 2 },
    { title: 'One-Click Run & Test Buttons', id: 'commands', level: 2 },
    { title: 'Snippets Library', id: 'snippets', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="overview" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Overview & Capabilities
      </h2>
      <p>
        The official Persica VS Code extension provides a rich developer experience tailored specifically for Persica’s syntax and workflow.
      </p>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 my-6 text-xs">
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <div className="font-bold text-emerald-400 text-sm mb-1">Editor Intelligence</div>
          <p className="text-slate-400">Syntax highlighting for all keywords, types, and prelude symbols, plus hover documentation and code completion.</p>
        </div>
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <div className="font-bold text-emerald-400 text-sm mb-1">Live Diagnostics</div>
          <p className="text-slate-400">Background validation on file save, highlighting syntax and static type mismatches in real-time.</p>
        </div>
      </div>

      <h2 id="installation" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Installing the Extension (.vsix)
      </h2>
      <p>
        The extension is distributed as a self-contained <code className="text-emerald-400">.vsix</code> package:
      </p>

      <div className="my-4">
        <a
          href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/persica-0.13.0.vsix"
          className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-600 text-white font-bold text-xs shadow-lg shadow-emerald-500/30 transition transform hover:-translate-y-0.5"
        >
          <Download className="w-4 h-4" />
          <span>Download persica-0.13.0.vsix</span>
        </a>
      </div>

      <p className="text-xs text-slate-400">Install via command line or VS Code UI:</p>
      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`code --install-extension persica-0.13.0.vsix`}
      />

      <p className="text-xs text-slate-400 mt-2">
        Alternatively in VS Code: Open the <strong>Extensions</strong> tab (<kbd className="bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+Shift+X</kbd>), click the three dots (<code className="text-slate-200">···</code>) at the top right, and choose <strong>"Install from VSIX..."</strong>.
      </p>

      <h2 id="intellisense" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        IntelliSense & Auto-Complete
      </h2>
      <p>
        As you type in any <code className="text-emerald-300">.persi</code> file, VS Code provides auto-completion for:
      </p>
      <ul className="text-xs text-slate-300 list-disc pl-5 space-y-1 my-3">
        <li>All built-in functions (<code className="text-emerald-400">print</code>, <code className="text-emerald-400">input</code>, <code className="text-emerald-400">len</code>, <code className="text-emerald-400">range</code>, <code className="text-emerald-400">append</code>, etc.) with signatures.</li>
        <li>Standard Prelude classes (<code className="text-emerald-400">Stack</code>, <code className="text-emerald-400">Queue</code>, <code className="text-emerald-400">Set</code>, <code className="text-emerald-400">StringBuilder</code>, <code className="text-emerald-400">Result</code>, <code className="text-emerald-400">Option</code>).</li>
        <li>Primitive and collection types (<code className="text-emerald-400">int</code>, <code className="text-emerald-400">float</code>, <code className="text-emerald-400">string</code>, <code className="text-emerald-400">bool</code>, <code className="text-emerald-400">void</code>, <code className="text-emerald-400">map</code>, <code className="text-emerald-400">dict</code>).</li>
        <li>Control-flow and declaration keywords.</li>
      </ul>

      <h2 id="hover" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Hover Documentation
      </h2>
      <p>
        Hover your cursor over any built-in symbol to view its complete function signature, parameter types, return values, and behavior notes in a formatted popup window.
      </p>

      <h2 id="diagnostics" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Live Error Diagnostics on Save
      </h2>
      <p>
        Every time you save a <code className="text-emerald-300">.persi</code> file, the extension automatically invokes <code className="text-emerald-300">persica --check</code> in the background. If any syntax error or static type mismatch is detected, it is immediately underlined in red in the editor and listed in the <strong>Problems panel</strong> (<kbd className="bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+Shift+M</kbd>).
      </p>

      <h2 id="commands" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        One-Click Run & Test Buttons
      </h2>
      <p>
        The extension registers dedicated buttons directly in the top-right editor title bar:
      </p>
      <ul className="text-xs text-slate-300 list-disc pl-5 space-y-1 my-3">
        <li><strong>Play Button (<code className="text-emerald-400">$(play)</code>):</strong> Runs the active file in an integrated Persica terminal.</li>
        <li><strong>Check Button (<code className="text-emerald-400">$(check)</code>):</strong> Manually runs syntax & semantic checks on the current file.</li>
      </ul>

      <p className="text-xs text-slate-400 mt-2">Available in the Command Palette (<kbd className="bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+Shift+P</kbd>):</p>
      <div className="p-3 rounded-xl border border-slate-800 bg-slate-900/60 font-mono text-xs text-slate-300 my-2 space-y-1">
        <div>&gt; Persica: Run Current File</div>
        <div>&gt; Persica: Check Syntax & Types</div>
        <div>&gt; Persica: Run Project (persica run)</div>
        <div>&gt; Persica: Run Tests (persica test)</div>
        <div>&gt; Persica: Create New Project (persica new)</div>
      </div>
    </DocsLayout>
  );
}
