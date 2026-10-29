import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { Terminal, Shield, Command } from 'lucide-react';

export default function CliReferencePage() {
  const meta = {
    title: 'CLI Commands Reference',
    category: 'Tooling & Ecosystem',
    description: 'Complete manual for all command-line interface arguments, subcommands, flags, and diagnostic modes in the Persica compiler.',
  };

  const toc = [
    { title: 'Command Overview', id: 'overview', level: 2 },
    { title: 'Project Management Subcommands', id: 'project-commands', level: 2 },
    { title: 'Direct Execution Modes', id: 'execution-modes', level: 2 },
    { title: 'Static Analysis & Inspection Flags', id: 'inspection-flags', level: 2 },
    { title: 'Standard Input (Pipelines)', id: 'stdin', level: 2 },
    { title: 'Exit Codes & Diagnostics', id: 'exit-codes', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="overview" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Command Overview
      </h2>
      <p>
        The <code className="text-emerald-400 font-mono">persica</code> binary is a single self-contained executable that handles compiling, interpreting, project scaffolding, unit testing, and static analysis:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`persica [SUBCOMMAND] [OPTIONS] [FILE]`}
      />

      <div className="overflow-x-auto my-4 text-xs">
        <table className="w-full text-left border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">Command</th>
              <th className="p-3">Description</th>
              <th className="p-3">Example Usage</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica &lt;file.persi&gt;</td>
              <td className="p-3">Execute a standalone single file</td>
              <td className="p-3 font-mono">persica app.persi</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica new &lt;name&gt;</td>
              <td className="p-3">Scaffold a complete new project</td>
              <td className="p-3 font-mono">persica new web_api</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica init</td>
              <td className="p-3">Initialize current directory as project</td>
              <td className="p-3 font-mono">persica init</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica run</td>
              <td className="p-3">Run the project entrypoint in persica.toml</td>
              <td className="p-3 font-mono">persica run</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica test</td>
              <td className="p-3">Run all unit tests in tests/ directory</td>
              <td className="p-3 font-mono">persica test</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica check &lt;file&gt;</td>
              <td className="p-3">Validate syntax & types without running</td>
              <td className="p-3 font-mono">persica check src/main.persi</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica --eval "&lt;code&gt;"</td>
              <td className="p-3">Evaluate an inline snippet of code</td>
              <td className="p-3 font-mono">persica --eval "print(2+2);"</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">persica -</td>
              <td className="p-3">Read program source from standard input</td>
              <td className="p-3 font-mono">cat main.persi | persica -</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2 id="project-commands" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Project Management Subcommands
      </h2>

      <div className="space-y-6 my-4">
        <div>
          <h4 className="text-sm font-bold text-white mb-1"><code className="text-emerald-400 font-mono">persica new &lt;name&gt;</code></h4>
          <p className="text-xs text-slate-400 mb-2">Creates a new directory with a configured <code className="text-slate-300">persica.toml</code>, <code className="text-slate-300">src/main.persi</code>, and <code className="text-slate-300">.persi_env/</code> local dependencies folder.</p>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white mb-1"><code className="text-emerald-400 font-mono">persica run</code></h4>
          <p className="text-xs text-slate-400 mb-2">Searches for <code className="text-slate-300">persica.toml</code> in the current folder or any ancestor directory, configures the module resolution paths, and executes the designated entrypoint.</p>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white mb-1"><code className="text-emerald-400 font-mono">persica test</code></h4>
          <p className="text-xs text-slate-400 mb-2">Scans the <code className="text-slate-300">tests/</code> folder and executes every <code className="text-slate-300">*.persi</code> test file, reporting individual pass/fail results and an exit code for CI systems.</p>
        </div>
      </div>

      <h2 id="inspection-flags" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Static Analysis & Inspection Flags
      </h2>
      <p>
        Persica includes compiler diagnostics useful for tooling and editor integration:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`# 1. Type check without execution
persica check main.persi
# Output: "check succeeded" (exit code 0)

# 2. Dump lexer tokens
persica --tokens main.persi

# 3. Dump parsed Abstract Syntax Tree (AST)
persica --ast main.persi`}
      />

      <h2 id="stdin" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Standard Input (Pipelines)
      </h2>
      <p>
        Pass a single dash <code className="text-emerald-400 font-bold">-</code> to execute code piped from another shell command:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`echo 'print("Piped execution!");' | persica -`}
        output={`Piped execution!`}
      />

      <h2 id="exit-codes" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Exit Codes & Diagnostics
      </h2>
      <p>
        Persica communicates cleanly with scripts and CI pipelines via standard exit codes:
      </p>
      <ul className="text-xs text-slate-300 list-disc pl-5 space-y-1 my-3">
        <li><code className="text-emerald-400">0</code> — Success (execution or static check passed cleanly).</li>
        <li><code className="text-rose-400">1</code> — Compile or runtime diagnostic error. Details are printed to <code className="text-slate-300">stderr</code> in source-aware format: <code className="text-slate-300">error[phase]: message --&gt; file:line:col</code>.</li>
      </ul>
    </DocsLayout>
  );
}
