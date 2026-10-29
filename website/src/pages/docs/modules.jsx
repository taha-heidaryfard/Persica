import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';

export default function ModulesPage() {
  const meta = {
    title: 'Pythonic Modules & Imports',
    category: 'Object-Oriented & Robustness',
    description: 'Structure multi-file codebases with Persica’s Python-style import system, folder packages, aliasing, and circular import protection.',
  };

  const toc = [
    { title: 'The Import Philosophy', id: 'philosophy', level: 2 },
    { title: 'Import Styles & Syntax', id: 'syntax', level: 2 },
    { title: 'Folder Modules (`mod.persi`)', id: 'folders', level: 2 },
    { title: 'Submodule Dot Notation', id: 'submodules', level: 2 },
    { title: 'Circular Import Protection', id: 'circular', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="philosophy" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        The Import Philosophy
      </h2>
      <p>
        Persica adopts the clean, elegant import syntax of Python:
      </p>
      <ul className="text-xs text-slate-300 list-disc pl-5 space-y-1 my-3">
        <li><strong>No File Extensions:</strong> Write <code className="text-emerald-400">import math;</code> rather than quoting <code className="text-slate-400">"math.persi"</code>.</li>
        <li><strong>Public by Default:</strong> All top-level functions, structs, classes, and constants in a module are available to importers.</li>
        <li><strong>Compile-Time Caching:</strong> Each module is evaluated only once; repeated imports reuse the compiled module environment.</li>
      </ul>

      <h2 id="syntax" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Import Styles & Syntax
      </h2>

      <div className="space-y-6 my-4">
        <div>
          <h4 className="text-sm font-bold text-white">1. Direct Module Import</h4>
          <p className="text-xs text-slate-400 mb-2">Access symbols through the module namespace using dot syntax:</p>
          <CodeBlock
            language="persica"
            filename="app.persi"
            code={`import math;

print("Square root of 49.0:", math.sqrt(49.0));`}
            output={`Square root of 49.0: 7`}
          />
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">2. Module Aliasing (`as`)</h4>
          <CodeBlock
            language="persica"
            filename="app.persi"
            code={`import math as m;

print("Cosine of 0.0:", m.cos(0.0));`}
            output={`Cosine of 0.0: 1`}
          />
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">3. Specific Symbol Imports</h4>
          <p className="text-xs text-slate-400 mb-2">Import specific items directly into the current scope:</p>
          <CodeBlock
            language="persica"
            filename="app.persi"
            code={`from models import Point, User;
from math import sqrt as square_root;

Point p = Point(10, 20);
print("Direct access to sqrt:", square_root(25.0));`}
          />
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">4. Wildcard Imports (`*`)</h4>
          <CodeBlock
            language="persica"
            filename="app.persi"
            code={`from utils.formatters import *;

// All symbols from formatters are now directly accessible
print(format_currency(1500));`}
          />
        </div>
      </div>

      <h2 id="folders" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Folder Modules (`mod.persi` & `index.persi`)
      </h2>
      <p>
        When a package consists of multiple internal files, group them in a folder and provide an entrypoint named <code className="text-emerald-400">mod.persi</code> or <code className="text-emerald-400">index.persi</code>:
      </p>

      <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 font-mono text-xs text-slate-300 my-3">
        <div>📁 src/</div>
        <div className="pl-4">├── 📁 geometry/</div>
        <div className="pl-8">├── 📄 circle.persi</div>
        <div className="pl-8">└── 📄 mod.persi &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// exports from circle.persi</span></div>
        <div className="pl-4">└── 📄 main.persi</div>
      </div>

      <p className="text-xs text-slate-400">Inside <code className="text-emerald-300">src/main.persi</code>:</p>
      <CodeBlock
        language="persica"
        filename="src/main.persi"
        code={`import geometry;
// The compiler resolves this automatically to geometry/mod.persi`}
      />

      <h2 id="submodules" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Submodule Dot Notation
      </h2>
      <p>
        You can navigate nested directories using dot syntax:
      </p>

      <CodeBlock
        language="persica"
        filename="app.persi"
        code={`from network.http.client import HttpClient;

HttpClient client = HttpClient();
print("Client initialized!");`}
      />

      <h2 id="circular" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Circular Import Protection
      </h2>
      <p>
        Persica’s semantic module graph analyzer tracks in-flight imports. If file <code className="text-slate-300">A</code> imports <code className="text-slate-300">B</code> and file <code className="text-slate-300">B</code> imports <code className="text-slate-300">A</code>, the compiler halts cleanly and pinpoints the cycle without infinite recursion or runtime stack overflow.
      </p>
    </DocsLayout>
  );
}
